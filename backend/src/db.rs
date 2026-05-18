use crate::models::{Availability, Herb, HerbSummary};
use anyhow::Result;
use serde_json::json;
use sqlx::{sqlite::SqliteQueryResult, Row, SqlitePool};

pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS herbs (
            id INTEGER PRIMARY KEY,
            common_name TEXT NOT NULL,
            latin_name TEXT NOT NULL,
            local_names_json TEXT NOT NULL,
            regions_json TEXT NOT NULL,
            plant_part TEXT NOT NULL,
            traditional_uses_json TEXT NOT NULL,
            symptom_tags_json TEXT NOT NULL,
            evidence_level TEXT NOT NULL,
            safety_summary TEXT NOT NULL,
            contraindications_json TEXT NOT NULL,
            interactions_json TEXT NOT NULL,
            preparation_note TEXT NOT NULL,
            use_guidance TEXT NOT NULL DEFAULT '',
            source_url TEXT NOT NULL,
            image_url TEXT NOT NULL DEFAULT '',
            image_source_url TEXT NOT NULL DEFAULT '',
            identification_features_json TEXT NOT NULL DEFAULT '[]',
            local_availability TEXT NOT NULL DEFAULT ''
        );
        "#,
    )
    .execute(pool)
    .await?;

    ensure_herb_column(pool, "image_url", "TEXT NOT NULL DEFAULT ''").await?;
    ensure_herb_column(pool, "use_guidance", "TEXT NOT NULL DEFAULT ''").await?;
    ensure_herb_column(pool, "image_source_url", "TEXT NOT NULL DEFAULT ''").await?;
    ensure_herb_column(
        pool,
        "identification_features_json",
        "TEXT NOT NULL DEFAULT '[]'",
    )
    .await?;
    ensure_herb_column(pool, "local_availability", "TEXT NOT NULL DEFAULT ''").await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS conditions (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            severity_class TEXT NOT NULL,
            symptom_tags_json TEXT NOT NULL,
            red_flags_json TEXT NOT NULL,
            safe_response_policy TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS herb_region_availability (
            id INTEGER PRIMARY KEY,
            herb_id INTEGER NOT NULL,
            country TEXT NOT NULL,
            region TEXT NOT NULL,
            confidence REAL NOT NULL,
            source TEXT NOT NULL,
            FOREIGN KEY (herb_id) REFERENCES herbs(id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS consultations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            location TEXT NOT NULL,
            user_input_json TEXT NOT NULL,
            triage_result_json TEXT NOT NULL,
            retrieved_herbs_json TEXT NOT NULL,
            gemma_response TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn ensure_herb_column(pool: &SqlitePool, name: &str, definition: &str) -> Result<()> {
    let columns = sqlx::query("PRAGMA table_info(herbs)")
        .fetch_all(pool)
        .await?;
    let exists = columns.iter().any(|row| {
        let column_name: String = row.get("name");
        column_name == name
    });
    if !exists {
        sqlx::query(&format!("ALTER TABLE herbs ADD COLUMN {name} {definition}"))
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn seed_db(pool: &SqlitePool) -> Result<()> {
    let herbs = vec![
        (
            1,
            "Ginger",
            "Zingiber officinale",
            json!(["gung", "adrak"]),
            json!(["Vietnam", "India", "Global tropics"]),
            "rhizome",
            json!(["traditionally used for mild nausea, indigestion, cough comfort"]),
            json!(["mild nausea", "mild indigestion", "mild cough"]),
            "moderate",
            "Usually food-like in small culinary amounts; may irritate reflux in some people.",
            json!([
                "Use caution with gallbladder disease",
                "Use caution in pregnancy unless guided by clinician"
            ]),
            json!(["May increase bleeding risk with anticoagulants or antiplatelet medicines"]),
            "Use as a food or mild tea tradition; avoid concentrated dosing advice.",
            "https://www.ncbi.nlm.nih.gov/books/NBK92775/",
        ),
        (
            2,
            "Perilla",
            "Perilla frutescens",
            json!(["tia to", "shiso"]),
            json!(["Vietnam", "Korea", "Japan", "China"]),
            "leaf",
            json!(["traditionally used for mild cold symptoms and digestive comfort"]),
            json!(["runny nose", "mild cough", "mild indigestion"]),
            "limited",
            "Generally used as food; allergy is possible.",
            json!(["Avoid if allergic to mint-family plants"]),
            json!([]),
            "Discuss as a culinary leaf or gentle infusion tradition, not as a cure.",
            "https://powo.science.kew.org/",
        ),
        (
            3,
            "Vietnamese balm",
            "Elsholtzia ciliata",
            json!(["kinh gioi", "xiang ru"]),
            json!(["Vietnam", "China", "Korea"]),
            "aerial parts",
            json!(["traditionally used for mild colds and chills"]),
            json!(["runny nose", "mild cough", "mild sore throat"]),
            "traditional",
            "Traditional use data is stronger than clinical evidence; allergy possible.",
            json!(["Avoid during pregnancy unless a clinician says it is safe"]),
            json!([]),
            "Mention only traditional mild symptom support; no dosing.",
            "https://www.worldfloraonline.org/",
        ),
        (
            4,
            "Indian borage",
            "Coleus amboinicus",
            json!(["hung chanh", "panikoorka", "ajwain leaf", "oregano brujo"]),
            json!(["Vietnam", "India", "Caribbean", "Southeast Asia"]),
            "leaf",
            json!(["traditionally used for mild cough and throat comfort"]),
            json!(["mild cough", "mild sore throat"]),
            "traditional",
            "Aromatic leaf used in folk traditions; can irritate sensitive stomachs.",
            json!([
                "Avoid if allergic to Lamiaceae/mint-family plants",
                "Use caution in pregnancy"
            ]),
            json!([]),
            "Use cautious language about traditional leaf preparations only.",
            "https://powo.science.kew.org/",
        ),
        (
            5,
            "Turmeric",
            "Curcuma longa",
            json!(["nghe", "haldi"]),
            json!(["Vietnam", "India", "Southeast Asia"]),
            "rhizome",
            json!(["traditionally used for digestive comfort and skin support"]),
            json!(["mild indigestion", "mild skin irritation"]),
            "limited",
            "Culinary use is common; concentrated products may carry more risk.",
            json!([
                "Use caution with gallbladder disease",
                "Avoid applying to deep, infected, or burned skin"
            ]),
            json!(["May interact with anticoagulants or diabetes medicines"]),
            "Discuss food-level use and patch-test caution for skin traditions.",
            "https://www.nccih.nih.gov/health/turmeric",
        ),
        (
            6,
            "Peppermint",
            "Mentha x piperita",
            json!(["bac ha", "peppermint"]),
            json!(["Global temperate", "Vietnam urban markets"]),
            "leaf",
            json!(["traditionally used for mild indigestion and nausea"]),
            json!(["mild indigestion", "mild nausea"]),
            "moderate",
            "Can worsen reflux; essential oil should not be swallowed casually.",
            json!([
                "Avoid peppermint oil in young children",
                "Use caution with GERD/reflux"
            ]),
            json!(["May affect absorption timing of some medicines"]),
            "Leaf tea traditions only; avoid essential oil dosing.",
            "https://www.nccih.nih.gov/health/peppermint-oil",
        ),
        (
            7,
            "Chamomile",
            "Matricaria chamomilla",
            json!(["cuc la ma", "chamomile"]),
            json!(["Europe", "Global markets"]),
            "flower",
            json!(["traditionally used for stress and sleep support"]),
            json!(["stress/sleep support", "mild indigestion"]),
            "limited",
            "Allergy can occur, especially with ragweed-family sensitivity.",
            json!([
                "Avoid if allergic to Asteraceae/ragweed-family plants",
                "Use caution in pregnancy"
            ]),
            json!([
                "May add to sedative effects",
                "Use caution with anticoagulants"
            ]),
            "Mention gentle tea tradition without exact medicinal dosing.",
            "https://www.nccih.nih.gov/health/chamomile",
        ),
        (
            8,
            "Aloe vera",
            "Aloe barbadensis miller",
            json!(["nha dam", "aloe"]),
            json!(["Vietnam", "Mexico", "Global tropics"]),
            "leaf gel",
            json!(["traditionally used topically for minor skin irritation"]),
            json!(["mild skin irritation"]),
            "limited",
            "Topical gel may irritate some skin; oral latex can be unsafe.",
            json!([
                "Do not use on deep wounds, severe burns, or infected skin",
                "Avoid oral aloe latex"
            ]),
            json!(["Oral aloe may interact with diabetes medicines and diuretics"]),
            "Discuss external gel traditions only for minor irritation.",
            "https://www.nccih.nih.gov/health/aloe-vera",
        ),
        (
            9,
            "Guava leaf",
            "Psidium guajava",
            json!(["la oi", "guava leaf"]),
            json!(["Vietnam", "Mexico", "Tropics"]),
            "leaf",
            json!(["traditionally used for mild diarrhea support"]),
            json!(["mild diarrhea"]),
            "limited",
            "Seek care quickly for dehydration, blood in stool, fever, or infants.",
            json!(["Avoid delaying care for children, older adults, or severe diarrhea"]),
            json!(["May affect blood sugar; use caution with diabetes medicines"]),
            "No exact dosing; focus on hydration and safety boundaries.",
            "https://www.ncbi.nlm.nih.gov/pmc/",
        ),
        (
            10,
            "Houttuynia",
            "Houttuynia cordata",
            json!(["diep ca", "fish mint"]),
            json!(["Vietnam", "China", "Japan", "Korea"]),
            "leaf",
            json!(["traditionally used for mild throat and respiratory comfort"]),
            json!(["mild sore throat", "mild cough", "runny nose"]),
            "traditional",
            "Food and folk use exist; allergy or stomach upset possible.",
            json!(["Avoid if allergic or if symptoms are severe"]),
            json!([]),
            "Discuss as local food/herb tradition only.",
            "https://www.worldfloraonline.org/",
        ),
        (
            11,
            "Moringa",
            "Moringa oleifera",
            json!(["zogale", "moringa"]),
            json!(["Nigeria", "Sahel", "Global tropics"]),
            "leaf",
            json!(["traditionally used as a nutritious leaf food during recovery"]),
            json!(["recovery nutrition", "mild diarrhea", "abdominal discomfort", "food source", "long-term food", "moringa"]),
            "limited",
            "Commonly eaten as a leafy vegetable; supplements are different from food use.",
            json!(["Use caution in pregnancy with concentrated extracts"]),
            json!(["May affect blood sugar or blood pressure medicines"]),
            "Discuss as local food-level nutrition support, not as a treatment.",
            "https://powo.science.kew.org/",
        ),
        (
            12,
            "Bitter leaf",
            "Vernonia amygdalina",
            json!(["ewuro", "onugbu", "shuwaka", "bitter leaf"]),
            json!(["Nigeria", "West Africa"]),
            "leaf",
            json!(["traditionally used for digestive comfort in West African food and herbal traditions"]),
            json!(["abdominal discomfort", "mild indigestion", "recovery nutrition"]),
            "traditional",
            "Traditional use is common, but clinical evidence is limited; bitterness can upset the stomach.",
            json!(["Avoid concentrated preparations in pregnancy or young children unless guided by a clinician"]),
            json!(["May affect blood sugar medicines"]),
            "Mention food-level or gentle traditional use only; avoid dosing and cure claims.",
            "https://powo.science.kew.org/",
        ),
        (
            13,
            "Guava leaf",
            "Psidium guajava",
            json!(["guava leaf"]),
            json!(["Nigeria", "West Africa", "Global tropics"]),
            "leaf",
            json!(["traditionally used for mild diarrhea support"]),
            json!(["mild diarrhea", "loose stool"]),
            "limited",
            "Hydration is the priority; seek care for dehydration, blood in stool, fever, or children who cannot drink.",
            json!(["Do not use to delay ORS, zinc, antibiotics when prescribed, or urgent care"]),
            json!(["May affect blood sugar; use caution with diabetes medicines"]),
            "No exact dosing; discuss only as a local tradition alongside ORS and safe water.",
            "https://www.ncbi.nlm.nih.gov/pmc/",
        ),
        (
            14,
            "Orange-fleshed sweet potato",
            "Ipomoea batatas",
            json!(["dankali", "sweet potato"]),
            json!(["Nigeria", "West Africa", "Global tropics"]),
            "tuber and leaf",
            json!(["grown as a long-term household food crop and leafy vegetable"]),
            json!(["food source", "long-term food", "nutrition", "hunger"]),
            "moderate",
            "A food crop rather than a medicine; food safety and clean preparation matter.",
            json!(["Do not eat moldy, spoiled, or pesticide-contaminated tubers or leaves"]),
            json!([]),
            "Discuss as a resilient food crop for nutrition planning, not as a treatment.",
            "https://www.fao.org/",
        ),
        (
            15,
            "Neem",
            "Azadirachta indica",
            json!(["dogonyaro", "neem"]),
            json!(["Nigeria", "India", "Sahel", "Tropics"]),
            "leaf",
            json!(["traditionally used externally for mild skin comfort and household pest control"]),
            json!(["mild skin irritation", "insect bite", "mosquito prevention", "hygiene"]),
            "traditional",
            "Neem oil and concentrated neem products can be unsafe, especially for children and pregnancy.",
            json!(["Do not swallow neem oil or concentrated extracts", "Avoid in pregnancy", "Do not use on infants"]),
            json!(["May affect diabetes medicines or immune medicines"]),
            "Discuss only external, familiar, low-strength traditional use or shade/pest-control context.",
            "https://powo.science.kew.org/",
        ),
        (
            16,
            "Lemongrass",
            "Cymbopogon citratus",
            json!(["fever grass", "lemongrass"]),
            json!(["Nigeria", "West Africa", "Global tropics"]),
            "leaf",
            json!(["traditionally used as a mild tea for comfort during cough or stomach upset"]),
            json!(["mild cough", "mild sore throat", "mild indigestion", "nausea"]),
            "limited",
            "Food-like leaf tea is usually gentler than essential oil; essential oil should not be swallowed casually.",
            json!(["Avoid essential oil dosing", "Use caution in pregnancy", "Stop if rash or stomach upset occurs"]),
            json!(["May add to sedative effects or affect blood pressure in sensitive people"]),
            "Discuss as a weak familiar leaf tea only, not as fever or malaria treatment.",
            "https://powo.science.kew.org/taxon/396896-1",
        ),
        (
            17,
            "Roselle",
            "Hibiscus sabdariffa",
            json!(["zobo", "roselle", "hibiscus"]),
            json!(["Nigeria", "West Africa", "Global tropics"]),
            "calyx",
            json!(["used as a sour local drink and hydration support"]),
            json!(["hydration", "recovery nutrition", "mild sore throat", "food source"]),
            "limited",
            "A familiar drink for many families; very sweet drinks can worsen dental and blood sugar concerns.",
            json!(["Use caution with low blood pressure", "Use caution with diabetes medicines"]),
            json!(["May affect blood pressure or blood sugar medicines"]),
            "Discuss as a clean, lightly sweetened drink if water is safe; not a medicine.",
            "https://powo.science.kew.org/",
        ),
        (
            18,
            "Baobab",
            "Adansonia digitata",
            json!(["kuka", "baobab"]),
            json!(["Nigeria", "Sahel", "Africa"]),
            "fruit pulp and leaf",
            json!(["used as local food for nutrition support"]),
            json!(["recovery nutrition", "food source", "long-term food", "mild diarrhea"]),
            "limited",
            "Food use is common; cleanliness and safe storage matter.",
            json!(["Do not use moldy or contaminated powder"]),
            json!([]),
            "Discuss as food-level nutrition support from clean known sources.",
            "https://powo.science.kew.org/",
        ),
        (
            19,
            "Okra",
            "Abelmoschus esculentus",
            json!(["kubewa", "okra"]),
            json!(["Nigeria", "West Africa", "Global tropics"]),
            "pod and leaf",
            json!(["grown as a household vegetable and soft food"]),
            json!(["food source", "long-term food", "recovery nutrition", "mild indigestion"]),
            "limited",
            "A common food vegetable; wash and cook well.",
            json!(["Avoid spoiled or pesticide-contaminated pods"]),
            json!([]),
            "Discuss as a cooked vegetable and garden crop, not as a disease treatment.",
            "https://powo.science.kew.org/",
        ),
    ];

    for h in herbs {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO herbs
            (id, common_name, latin_name, local_names_json, regions_json, plant_part, traditional_uses_json,
             symptom_tags_json, evidence_level, safety_summary, contraindications_json, interactions_json,
             preparation_note, source_url)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(h.0)
        .bind(h.1)
        .bind(h.2)
        .bind(h.3.to_string())
        .bind(h.4.to_string())
        .bind(h.5)
        .bind(h.6.to_string())
        .bind(h.7.to_string())
        .bind(h.8)
        .bind(h.9)
        .bind(h.10.to_string())
        .bind(h.11.to_string())
        .bind(h.12)
        .bind(h.13)
        .execute(pool)
        .await?;
    }

    let herb_details = vec![
        (
            11,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Moringa%20leaf%20closeup.jpg?width=640",
            "https://commons.wikimedia.org/wiki/Category:Moringa_oleifera_(leaves)",
            json!([
                "Small oval leaflets arranged in feathery compound leaves",
                "Leaves grow on thin branching stems; young leaves are often harvested as a vegetable",
                "Do not identify from leaves alone if there are similar local trees nearby"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a regional leafy food; ask a local health worker, market vendor, or experienced grower before use.",
        ),
        (
            12,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Vernonia%20amygdalina%20aka%20Bitter%20leaf.jpg?width=640",
            "https://commons.wikimedia.org/wiki/File:Vernonia_amygdalina_aka_Bitter_leaf.jpg",
            json!([
                "Shrubby plant commonly known locally as bitter leaf",
                "Leaves are green, simple, and noticeably bitter when prepared as food",
                "Confirm with a trusted local identifier; do not use unknown wild plants for a child"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a West African household herb; easiest path is a known market source rather than foraging.",
        ),
        (
            13,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Psidium%20guajava%20leaves%20LR.jpg?width=640",
            "https://commons.wikimedia.org/wiki/File:Psidium_guajava_leaves_LR.jpg",
            json!([
                "Guava tree leaves are opposite, oval, and have many visible side veins",
                "Often found on cultivated guava trees near homes, markets, or gardens",
                "Use only leaves from a tree confidently identified as guava and not sprayed with chemicals"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a tropical fruit tree; look for cultivated trees or market leaves, not unknown roadside plants.",
        ),
        (
            14,
            "https://commons.wikimedia.org/wiki/Special:FilePath/20230227%20161110%20Ipomoea%20batatas%20%27Beauregard%27.jpg?width=640",
            "https://commons.wikimedia.org/wiki/File:20230227_161110_Ipomoea_batatas_%27Beauregard%27.jpg",
            json!([
                "Creeping vine with heart-shaped or lobed leaves",
                "Edible tubers grow underground and may be white, yellow, orange, or purple",
                "Use known planting material from farmers or markets rather than unknown wild vines"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a household food crop option; ask local growers which variety survives best in the season.",
        ),
        (
            15,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Azadirachta%20indica%20leaves.jpg?width=640",
            "https://commons.wikimedia.org/wiki/Category:Azadirachta_indica",
            json!([
                "Tree with many small pointed leaflets on each leaf stem",
                "Leaves are bitter and the tree is often used for shade in dry areas",
                "Do not use unknown seeds or oil; neem oil can be dangerous if swallowed"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a common shade and household tree; confirm with a local elder, grower, or health worker before use.",
        ),
        (
            16,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Cymbopogon%20citratus.jpg?width=640",
            "https://commons.wikimedia.org/wiki/Category:Cymbopogon_citratus",
            json!([
                "Tall clumping grass with long narrow leaves",
                "Leaves smell lemon-like when crushed",
                "Do not confuse with unknown sharp grasses or citronella products"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a cultivated household herb; safest source is a known garden or market bundle.",
        ),
        (
            17,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Hibiscus%20sabdariffa-flower-yercaud-salem-India.JPG?width=640",
            "https://commons.wikimedia.org/wiki/File:Hibiscus_sabdariffa-flower-yercaud-salem-India.JPG",
            json!([
                "Plant produces red fleshy calyces used for zobo drink",
                "Leaves are lobed and flowers are pale with a darker center",
                "Use clean dried calyces from a trusted source"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a familiar zobo drink plant; use clean dried calyces from market or known growers.",
        ),
        (
            18,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Adansonia%20digitata%20fruit%20MHNT.jpg?width=640",
            "https://commons.wikimedia.org/wiki/File:Adansonia_digitata_fruit_MHNT.jpg",
            json!([
                "Large baobab tree with thick trunk",
                "Hard hanging fruits contain dry pale pulp",
                "Use clean stored pulp or leaf powder, not moldy material"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a Sahel food tree; use clean market powder or known harvested fruit.",
        ),
        (
            19,
            "https://commons.wikimedia.org/wiki/Special:FilePath/Abelmoschus%20esculentus%20fruit.jpg?width=640",
            "https://commons.wikimedia.org/wiki/Category:Abelmoschus_esculentus",
            json!([
                "Upright plant with lobed leaves and yellowish hibiscus-like flowers",
                "Green ribbed pods are harvested young for food",
                "Cook pods well and avoid spoiled or sprayed produce"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a common vegetable crop; start from known seeds or market pods.",
        ),
    ];

    for detail in herb_details {
        sqlx::query(
            r#"
            UPDATE herbs
            SET image_url = ?,
                image_source_url = ?,
                identification_features_json = ?,
                local_availability = ?
            WHERE id = ?
            "#,
        )
        .bind(detail.1)
        .bind(detail.2)
        .bind(detail.3.to_string())
        .bind(detail.4)
        .bind(detail.0)
        .execute(pool)
        .await?;
    }

    let use_guidance = vec![
        (
            11,
            "Use only as familiar food-level leafy nutrition, such as cooked leaves in a normal meal. Do not use concentrated extracts for a child or as treatment for diarrhea, malaria, or worms.",
        ),
        (
            12,
            "Use only as a familiar food or gentle traditional preparation if the family already knows it. Do not force bitter preparations into a sick child, and do not use it instead of proven medicine.",
        ),
        (
            13,
            "If the plant is confidently identified, discuss only a mild traditional leaf infusion alongside ORS and safe water. No exact medicinal dose is provided; ORS is the life-saving step.",
        ),
        (
            14,
            "For food security, plant known sweet potato cuttings in loose soil, keep vines watered until established, and harvest tubers when mature. Cook tubers or leaves thoroughly before eating.",
        ),
        (
            15,
            "Do not swallow neem oil, seeds, or concentrated extracts. For support, use only familiar external traditions such as shade, household pest control, or very mild skin-wash practices confirmed by a trusted local adult.",
        ),
        (
            16,
            "Use only as a weak familiar leaf tea or food flavor. Do not use essential oil by mouth. Do not use it to treat malaria, pneumonia, or severe fever.",
        ),
        (
            17,
            "Prepare only with safe water and clean cups. Use as a normal drink, not as medicine. Avoid making it very sugary for sick children.",
        ),
        (
            18,
            "Use clean baobab pulp or leaf powder as food-level nutrition support. Mix with safe water or food only if the powder is clean and not moldy.",
        ),
        (
            19,
            "Grow or buy as a common vegetable. Wash, cook, and eat as food. This helps meals, but it does not treat infection.",
        ),
    ];

    for guidance in use_guidance {
        sqlx::query("UPDATE herbs SET use_guidance = ? WHERE id = ?")
            .bind(guidance.1)
            .bind(guidance.0)
            .execute(pool)
            .await?;
    }

    sqlx::query(
        r#"
        UPDATE herbs
        SET common_name = 'Indian borage',
            local_names_json = ?
        WHERE id = 4
        "#,
    )
    .bind(json!(["hung chanh", "panikoorka", "ajwain leaf", "oregano brujo"]).to_string())
    .execute(pool)
    .await?;

    sqlx::query("UPDATE herbs SET symptom_tags_json = ? WHERE id = 11")
        .bind(
            json!([
                "recovery nutrition",
                "mild diarrhea",
                "abdominal discomfort",
                "food source",
                "long-term food",
                "moringa"
            ])
            .to_string(),
        )
        .execute(pool)
        .await?;

    let availability = vec![
        (
            1,
            1,
            "Vietnam",
            "Nghe An",
            0.95,
            "demo curated local market knowledge",
        ),
        (
            2,
            2,
            "Vietnam",
            "Nghe An",
            0.9,
            "demo curated local food herb",
        ),
        (3, 3, "Vietnam", "Nghe An", 0.88, "demo curated local herb"),
        (
            4,
            4,
            "Vietnam",
            "Nghe An",
            0.85,
            "demo curated household herb",
        ),
        (5, 5, "Vietnam", "Nghe An", 0.92, "demo curated local crop"),
        (
            6,
            8,
            "Vietnam",
            "Nghe An",
            0.8,
            "demo cultivated garden plant",
        ),
        (7, 9, "Vietnam", "Nghe An", 0.9, "demo tropical fruit tree"),
        (8, 10, "Vietnam", "Nghe An", 0.87, "demo local edible herb"),
        (
            9,
            4,
            "Mexico",
            "Yucatan",
            0.9,
            "demo regional household herb",
        ),
        (
            10,
            8,
            "Mexico",
            "Yucatan",
            0.93,
            "demo cultivated garden plant",
        ),
        (11, 9, "Mexico", "Yucatan", 0.91, "demo tropical fruit tree"),
        (
            12,
            6,
            "United States",
            "California",
            0.75,
            "demo common market herb",
        ),
        (
            13,
            7,
            "United States",
            "California",
            0.78,
            "demo common market herb",
        ),
        (14, 11, "Nigeria", "Kano", 0.92, "demo regional leafy food"),
        (
            15,
            12,
            "Nigeria",
            "Kano",
            0.88,
            "demo West African household herb",
        ),
        (16, 13, "Nigeria", "Kano", 0.86, "demo tropical fruit tree"),
        (17, 14, "Nigeria", "Kano", 0.84, "demo food crop option"),
        (18, 15, "Nigeria", "Kano", 0.9, "demo common shade tree"),
        (
            19,
            16,
            "Nigeria",
            "Kano",
            0.82,
            "demo cultivated household herb",
        ),
        (
            20,
            17,
            "Nigeria",
            "Kano",
            0.88,
            "demo familiar local drink plant",
        ),
        (21, 18, "Nigeria", "Kano", 0.8, "demo Sahel food tree"),
        (22, 19, "Nigeria", "Kano", 0.86, "demo vegetable crop"),
        (23, 1, "India", "Bihar", 0.9, "demo common market herb"),
        (24, 4, "India", "Bihar", 0.82, "demo Indian household herb"),
        (25, 5, "India", "Bihar", 0.88, "demo common kitchen spice"),
        (26, 15, "India", "Bihar", 0.8, "demo common shade tree"),
    ];

    for a in availability {
        sqlx::query(
            "INSERT OR IGNORE INTO herb_region_availability (id, herb_id, country, region, confidence, source) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(a.0)
        .bind(a.1)
        .bind(a.2)
        .bind(a.3)
        .bind(a.4)
        .bind(a.5)
        .execute(pool)
        .await?;
    }

    let conditions = vec![
        (
            1,
            "mild cough",
            "mild",
            json!(["mild cough"]),
            json!([]),
            "May discuss retrieved herbs for mild support with safety warnings.",
        ),
        (
            2,
            "chest pain",
            "emergency",
            json!(["chest pain"]),
            json!(["chest pain", "difficulty breathing"]),
            "Emergency guidance only. Do not suggest herbs.",
        ),
        (
            3,
            "cancer",
            "serious",
            json!(["suspected cancer"]),
            json!(["suspected cancer"]),
            "Do not discuss cures. Encourage clinical care; supportive-care discussion only.",
        ),
        (
            4,
            "malaria",
            "serious",
            json!(["fever", "chills", "malaria"]),
            json!(["confusion", "seizure", "difficulty breathing", "unable to drink"]),
            "Prompt malaria testing and antimalarial medicine are needed. Do not suggest herbs as malaria treatment.",
        ),
        (
            5,
            "diarrhea",
            "caution",
            json!(["mild diarrhea", "loose stool"]),
            json!(["blood in stool", "severe dehydration", "unable to drink"]),
            "Prioritize ORS, safe water, zinc where available, and urgent care for dehydration or blood in stool.",
        ),
        (
            6,
            "intestinal worms",
            "caution",
            json!(["worms", "abdominal discomfort"]),
            json!(["severe abdominal pain", "blood in stool", "severe malnutrition"]),
            "Encourage deworming through a clinic, school, or community health worker; herbs are not a substitute for anthelminthic medicine.",
        ),
    ];

    for c in conditions {
        sqlx::query(
            "INSERT OR IGNORE INTO conditions (id, name, severity_class, symptom_tags_json, red_flags_json, safe_response_policy) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(c.0)
        .bind(c.1)
        .bind(c.2)
        .bind(c.3.to_string())
        .bind(c.4.to_string())
        .bind(c.5)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn retrieve_herbs(
    pool: &SqlitePool,
    country: Option<&str>,
    region: Option<&str>,
    symptom: Option<&str>,
) -> Result<Vec<HerbSummary>> {
    let herbs = sqlx::query_as::<_, Herb>("SELECT * FROM herbs")
        .fetch_all(pool)
        .await?;
    let availability = sqlx::query_as::<_, Availability>("SELECT * FROM herb_region_availability")
        .fetch_all(pool)
        .await?;
    let symptom_lc = symptom.unwrap_or("").to_lowercase();
    let country_lc = country.unwrap_or("").to_lowercase();
    let region_lc = region.unwrap_or("").to_lowercase();

    let mut scored = Vec::new();
    for herb in herbs {
        let tags = parse_vec(&herb.symptom_tags_json);
        let tag_match = symptom_lc.is_empty()
            || tags.iter().any(|tag| {
                symptom_lc.contains(&tag.to_lowercase()) || tag.to_lowercase().contains(&symptom_lc)
            });
        if !tag_match {
            continue;
        }

        let best_availability_record = availability
            .iter()
            .filter(|a| a.herb_id == herb.id)
            .filter(|a| country_lc.is_empty() || a.country.to_lowercase() == country_lc)
            .filter(|a| region_lc.is_empty() || a.region.to_lowercase() == region_lc)
            .max_by(|a, b| {
                a.confidence
                    .partial_cmp(&b.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        if !country_lc.is_empty() && best_availability_record.is_none() {
            continue;
        }
        let best_availability = best_availability_record
            .map(|a| a.confidence)
            .unwrap_or(0.45);
        let local_availability = best_availability_record
            .map(|a| {
                format!(
                    "Confirmed near the patient location in demo availability data: {}, {} ({:.0}% confidence, {}).",
                    a.region,
                    a.country,
                    a.confidence * 100.0,
                    a.source
                )
            })
            .filter(|_| !country_lc.is_empty())
            .unwrap_or_else(|| herb.local_availability.clone());

        let evidence_score = match herb.evidence_level.as_str() {
            "strong" => 4.0,
            "moderate" => 3.0,
            "limited" => 2.0,
            _ => 1.0,
        };
        let safety_score = if herb.safety_summary.to_lowercase().contains("unsafe") {
            0.5
        } else {
            1.5
        };
        let score = best_availability * 3.0 + evidence_score + safety_score;
        scored.push((
            score,
            to_summary(herb, &tags, &symptom_lc, local_availability),
        ));
    }

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    Ok(scored.into_iter().map(|(_, h)| h).take(5).collect())
}

pub async fn save_consultation(
    pool: &SqlitePool,
    location: &str,
    user_input_json: &str,
    triage_result_json: &str,
    retrieved_herbs_json: &str,
    gemma_response: &str,
) -> Result<SqliteQueryResult> {
    Ok(sqlx::query(
        r#"
        INSERT INTO consultations
        (location, user_input_json, triage_result_json, retrieved_herbs_json, gemma_response)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(location)
    .bind(user_input_json)
    .bind(triage_result_json)
    .bind(retrieved_herbs_json)
    .bind(gemma_response)
    .execute(pool)
    .await?)
}

fn to_summary(
    herb: Herb,
    tags: &[String],
    symptom: &str,
    local_availability: String,
) -> HerbSummary {
    let matched = tags
        .iter()
        .find(|tag| symptom.contains(&tag.to_lowercase()))
        .cloned()
        .unwrap_or_else(|| {
            tags.first()
                .cloned()
                .unwrap_or_else(|| "mild symptom support".to_string())
        });
    HerbSummary {
        id: herb.id,
        common_name: herb.common_name,
        latin_name: herb.latin_name,
        why_relevant: format!("Retrieved because demo records link it with {matched}."),
        evidence_level: herb.evidence_level,
        safety_summary: herb.safety_summary,
        contraindications: parse_vec(&herb.contraindications_json),
        interactions: parse_vec(&herb.interactions_json),
        preparation_note: herb.preparation_note,
        use_guidance: herb.use_guidance,
        source_url: herb.source_url,
        image_url: herb.image_url,
        image_source_url: herb.image_source_url,
        identification_features: parse_vec(&herb.identification_features_json),
        local_availability,
    }
}

pub fn parse_vec(raw: &str) -> Vec<String> {
    serde_json::from_str(raw).unwrap_or_default()
}
