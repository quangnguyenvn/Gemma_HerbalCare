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
        (
            20,
            "Holy basil",
            "Ocimum tenuiflorum",
            json!(["tulsi", "holy basil"]),
            json!(["India", "South Asia", "Global tropics"]),
            "leaf",
            json!(["traditionally used for mild cough, throat comfort, and household wellness"]),
            json!(["mild cough", "mild sore throat", "runny nose", "stress/sleep support"]),
            "traditional",
            "A familiar household herb in India; avoid concentrated products for pregnancy or children unless guided by a clinician.",
            json!(["Use caution in pregnancy", "Avoid if allergic to mint-family plants"]),
            json!(["May affect blood sugar medicines or anticoagulants"]),
            "Discuss only familiar leaf tea or food-level tradition, not as treatment for severe fever or malaria.",
            "https://powo.science.kew.org/",
        ),
        (
            21,
            "Amla",
            "Phyllanthus emblica",
            json!(["amla", "Indian gooseberry"]),
            json!(["India", "South Asia"]),
            "fruit",
            json!(["used as a sour food fruit and traditional nutrition support"]),
            json!(["recovery nutrition", "food source", "mild sore throat"]),
            "limited",
            "Usually food-like as fruit; sour preparations can irritate sensitive stomachs.",
            json!(["Use caution with severe acidity or stomach ulcer symptoms"]),
            json!(["May affect blood sugar medicines in sensitive people"]),
            "Discuss as clean food-level fruit or familiar preparation only.",
            "https://powo.science.kew.org/",
        ),
        (
            22,
            "Coriander",
            "Coriandrum sativum",
            json!(["dhania", "coriander"]),
            json!(["India", "Global markets"]),
            "leaf and seed",
            json!(["used as a food herb for flavor and mild digestive comfort"]),
            json!(["mild indigestion", "nausea", "food source"]),
            "limited",
            "Common food herb; allergy is possible.",
            json!(["Avoid if allergic to coriander or related plants"]),
            json!(["May affect blood sugar in large amounts"]),
            "Use as normal food seasoning; do not use concentrated dosing.",
            "https://powo.science.kew.org/",
        ),
        (
            23,
            "Fennel",
            "Foeniculum vulgare",
            json!(["saunf", "fennel"]),
            json!(["India", "Global markets"]),
            "seed",
            json!(["traditionally used after meals for mild digestive comfort"]),
            json!(["mild indigestion", "nausea", "abdominal discomfort"]),
            "limited",
            "Food-level use is common; concentrated oils are different and can be risky.",
            json!(["Avoid essential oil dosing", "Use caution in pregnancy"]),
            json!(["May interact with hormone-sensitive conditions or medicines"]),
            "Discuss as familiar food-level seed use only.",
            "https://powo.science.kew.org/",
        ),
        (
            24,
            "Cumin",
            "Cuminum cyminum",
            json!(["jeera", "cumin"]),
            json!(["India", "Global markets"]),
            "seed",
            json!(["used as a kitchen spice and mild digestive tradition"]),
            json!(["mild indigestion", "abdominal discomfort", "food source"]),
            "traditional",
            "Common kitchen spice; high-dose products are different from food use.",
            json!(["Use caution with pregnancy if using concentrated products"]),
            json!(["May affect blood sugar medicines in large amounts"]),
            "Use food-level language only; avoid medicinal dosing.",
            "https://powo.science.kew.org/",
        ),
        (
            25,
            "Fenugreek",
            "Trigonella foenum-graecum",
            json!(["methi", "fenugreek"]),
            json!(["India", "South Asia", "Global markets"]),
            "leaf and seed",
            json!(["used as food leaves and seeds in traditional cooking"]),
            json!(["food source", "recovery nutrition", "mild indigestion"]),
            "limited",
            "Food use is common; seed supplements can affect blood sugar and pregnancy safety.",
            json!(["Avoid concentrated seed use in pregnancy", "Use caution with diabetes"]),
            json!(["May lower blood sugar or interact with anticoagulants"]),
            "Discuss cooked leaves or food-level seed use, not supplement dosing.",
            "https://powo.science.kew.org/",
        ),
        (
            26,
            "Garlic",
            "Allium sativum",
            json!(["lahsun", "garlic"]),
            json!(["India", "Global markets"]),
            "bulb",
            json!(["used as food and traditional support for mild cold-season comfort"]),
            json!(["mild cough", "food source", "recovery nutrition"]),
            "limited",
            "Food use is common; raw garlic can irritate the stomach and skin.",
            json!(["Use caution with stomach irritation", "Avoid applying raw garlic to skin"]),
            json!(["May increase bleeding risk with anticoagulants or antiplatelet medicines"]),
            "Discuss as cooked food seasoning only; avoid concentrated dosing.",
            "https://powo.science.kew.org/",
        ),
        (
            27,
            "Onion",
            "Allium cepa",
            json!(["pyaz", "onion"]),
            json!(["India", "Global markets"]),
            "bulb",
            json!(["used as a food vegetable and household cooking staple"]),
            json!(["food source", "recovery nutrition", "mild cough"]),
            "traditional",
            "A common food; can irritate reflux or sensitive stomachs.",
            json!(["Use caution with reflux or severe stomach irritation"]),
            json!([]),
            "Discuss as normal cooked food, not medicine.",
            "https://powo.science.kew.org/",
        ),
        (
            28,
            "Mint",
            "Mentha spicata",
            json!(["pudina", "mint"]),
            json!(["India", "Global markets"]),
            "leaf",
            json!(["used as food herb for cooling flavor and mild digestive comfort"]),
            json!(["mild indigestion", "nausea", "mild sore throat"]),
            "limited",
            "Leaf use is usually food-like; mint can worsen reflux in some people.",
            json!(["Use caution with GERD or reflux"]),
            json!(["May affect timing of some medicines if taken as concentrated products"]),
            "Use leaf chutney or mild tea tradition only; avoid essential oil dosing.",
            "https://powo.science.kew.org/",
        ),
        (
            29,
            "Curry leaf",
            "Murraya koenigii",
            json!(["kadi patta", "curry leaf"]),
            json!(["India", "South Asia"]),
            "leaf",
            json!(["used as a food leaf in cooking and household nutrition tradition"]),
            json!(["food source", "recovery nutrition", "mild indigestion"]),
            "traditional",
            "Commonly used in cooking; allergy is possible but uncommon.",
            json!(["Avoid if allergy occurs"]),
            json!([]),
            "Discuss as normal cooked food leaf, not as a disease treatment.",
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
            2,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a8/Perilla_leaves_2.jpg/960px-Perilla_leaves_2.jpg",
            "https://commons.wikimedia.org/wiki/File:Perilla_leaves_2.jpg",
            json!([
                "Broad green perilla leaves with toothed edges and a strong herbal smell",
                "Often sold as fresh leaves; confirm with a trusted local source before use",
                "Do not use unknown wild leaves for a sick child"
            ]),
            "Public plant photo shown for field recognition; confirm real plants with a trusted local identifier.",
        ),
        (
            1,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/3/3d/Ginger_rhizome.jpg/960px-Ginger_rhizome.jpg",
            "https://commons.wikimedia.org/wiki/Category:Zingiber_officinale_(root)",
            json!([
                "Ginger is the knobbly tan rhizome sold as a food spice",
                "Fresh pieces smell sharp, warm, and spicy when cut or scraped",
                "Do not confuse powdered spice or unknown roots with a fresh identified rhizome"
            ]),
            "Confirmed in demo availability data for Bihar, India as a common market food spice; safest source is a known food market, not foraging.",
        ),
        (
            4,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/9/9f/IndianBorage.jpeg/960px-IndianBorage.jpeg",
            "https://commons.wikimedia.org/wiki/File:IndianBorage.jpeg",
            json!([
                "Thick, soft, fuzzy leaves with rounded scalloped edges",
                "Leaves smell strong and oregano-like when gently crushed",
                "Confirm with a trusted local grower because mint-family plants can look similar"
            ]),
            "Confirmed in demo availability data for Bihar, India as an Indian household herb; safest source is a known garden, pot, or market bundle.",
        ),
        (
            5,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/5/5b/Curcuma_longa_roots.jpg/960px-Curcuma_longa_roots.jpg",
            "https://commons.wikimedia.org/wiki/File:Curcuma_longa_roots.jpg",
            json!([
                "Orange-yellow rhizome used as a kitchen spice",
                "Powder and fresh rhizome can look different",
                "Use known food-market turmeric, not unknown roots"
            ]),
            "Public plant photo shown for field recognition; safest source is a known market spice.",
        ),
        (
            6,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/f/f1/Peppermint_Plant.jpg/960px-Peppermint_Plant.jpg",
            "https://commons.wikimedia.org/wiki/File:Peppermint_Plant.jpg",
            json!([
                "Aromatic mint-family leaf",
                "Leaves smell minty when gently crushed",
                "Avoid essential oil dosing"
            ]),
            "Public plant photo shown for field recognition; use trusted market mint leaves.",
        ),
        (
            7,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/7/7f/Chamomile_flowers.jpg/960px-Chamomile_flowers.jpg",
            "https://commons.wikimedia.org/wiki/File:Chamomile_flowers.jpg",
            json!([
                "Small daisy-like flower heads in dried market form",
                "Do not confuse with unknown wild flowers",
                "Avoid if allergic to ragweed-family plants"
            ]),
            "Public plant photo shown for field recognition; use only trusted dried market material.",
        ),
        (
            8,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/0/01/Aloe_vera_leaf_showing_the_gel_(2).JPG/960px-Aloe_vera_leaf_showing_the_gel_(2).JPG",
            "https://commons.wikimedia.org/wiki/File:Aloe_vera_leaf_showing_the_gel_(2).JPG",
            json!([
                "Thick succulent leaves with clear inner gel",
                "Use external gel traditions only for minor irritation",
                "Do not swallow aloe latex"
            ]),
            "Public plant photo shown for field recognition; use known cultivated aloe only.",
        ),
        (
            9,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/4/45/Psidium_guajava_leaves_LR.jpg/960px-Psidium_guajava_leaves_LR.jpg",
            "https://commons.wikimedia.org/wiki/File:Psidium_guajava_leaves_LR.jpg",
            json!([
                "Cultivated guava tree leaves with many visible side veins",
                "Use only leaves from a known guava tree",
                "Avoid sprayed or roadside leaves"
            ]),
            "Public plant photo shown for field recognition; confirm with a known fruit tree or trusted market source.",
        ),
        (
            11,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/e/e3/Moringa_leaf_closeup.jpg/960px-Moringa_leaf_closeup.jpg",
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
            "https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Vernonia_amygdalina_aka_Bitter_leaf.jpg/960px-Vernonia_amygdalina_aka_Bitter_leaf.jpg",
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
            "https://upload.wikimedia.org/wikipedia/commons/thumb/4/45/Psidium_guajava_leaves_LR.jpg/960px-Psidium_guajava_leaves_LR.jpg",
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
            "https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/20230227_161110_Ipomoea_batatas_%27Beauregard%27.jpg/960px-20230227_161110_Ipomoea_batatas_%27Beauregard%27.jpg",
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
            "https://upload.wikimedia.org/wikipedia/commons/thumb/5/5e/Leaf_of_neem.jpg/960px-Leaf_of_neem.jpg",
            "https://commons.wikimedia.org/wiki/File:Leaf_of_neem.jpg",
            json!([
                "Tree with many small pointed leaflets on each leaf stem",
                "Leaves are bitter and the tree is often used for shade in dry areas",
                "Do not use unknown seeds or oil; neem oil can be dangerous if swallowed"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a common shade and household tree; confirm with a local elder, grower, or health worker before use.",
        ),
        (
            16,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Cymbopogon_citratus.jpg/960px-Cymbopogon_citratus.jpg",
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
            "https://upload.wikimedia.org/wikipedia/commons/thumb/2/2f/Hibiscus_sabdariffa-flower-yercaud-salem-India.JPG/960px-Hibiscus_sabdariffa-flower-yercaud-salem-India.JPG",
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
            "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d4/Adansonia_digitata.jpg/960px-Adansonia_digitata.jpg",
            "https://commons.wikimedia.org/wiki/File:Adansonia_digitata.jpg",
            json!([
                "Large baobab tree with thick trunk",
                "Hard hanging fruits contain dry pale pulp",
                "Use clean stored pulp or leaf powder, not moldy material"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a Sahel food tree; use clean market powder or known harvested fruit.",
        ),
        (
            19,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/b/b6/Okra_%28Abelmoschus_esculentus%29_fruit_opened.jpg/960px-Okra_%28Abelmoschus_esculentus%29_fruit_opened.jpg",
            "https://commons.wikimedia.org/wiki/File:Okra_(Abelmoschus_esculentus)_fruit_opened.jpg",
            json!([
                "Upright plant with lobed leaves and yellowish hibiscus-like flowers",
                "Green ribbed pods are harvested young for food",
                "Cook pods well and avoid spoiled or sprayed produce"
            ]),
            "Confirmed in demo availability data for Kano, Nigeria as a common vegetable crop; start from known seeds or market pods.",
        ),
        (
            20,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/2/27/(Ocimum_tenuiflorum)_Holy_Tulasi_plant_at_Kakinada_01.jpg/960px-(Ocimum_tenuiflorum)_Holy_Tulasi_plant_at_Kakinada_01.jpg",
            "https://commons.wikimedia.org/wiki/File:(Ocimum_tenuiflorum)_Holy_Tulasi_plant_at_Kakinada_01.jpg",
            json!([
                "Aromatic household basil plant",
                "Leaves grow opposite on square mint-family stems",
                "Confirm with a known household tulsi plant"
            ]),
            "Public plant photo shown for field recognition; safest source is a known home or market plant.",
        ),
        (
            21,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/0/02/Phyllanthus_emblica_BNC.jpg/960px-Phyllanthus_emblica_BNC.jpg",
            "https://commons.wikimedia.org/wiki/File:Phyllanthus_emblica_BNC.jpg",
            json!([
                "Small round sour green fruit",
                "Known locally as amla or Indian gooseberry",
                "Use clean food fruit from a trusted source"
            ]),
            "Public plant photo shown for field recognition; use clean market fruit.",
        ),
        (
            22,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/8/86/Coriander_Seeds.jpg/960px-Coriander_Seeds.jpg",
            "https://commons.wikimedia.org/wiki/File:Coriander_Seeds.jpg",
            json!([
                "Food herb with fresh leaves and small round seeds",
                "Known as dhania in many Indian markets",
                "Use normal food seasoning only"
            ]),
            "Public plant photo shown for field recognition; use trusted kitchen coriander.",
        ),
        (
            23,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/f/f2/Foeniculum_vulgare_seeds.jpg/960px-Foeniculum_vulgare_seeds.jpg",
            "https://commons.wikimedia.org/wiki/File:Foeniculum_vulgare_seeds.jpg",
            json!([
                "Small aromatic seeds used after meals",
                "Known as saunf in many Indian markets",
                "Avoid essential oil dosing"
            ]),
            "Public plant photo shown for field recognition; use trusted market seed.",
        ),
        (
            24,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/6/61/Whole_Cumin_Seeds.jpg/960px-Whole_Cumin_Seeds.jpg",
            "https://commons.wikimedia.org/wiki/File:Whole_Cumin_Seeds.jpg",
            json!([
                "Small brownish kitchen seeds",
                "Known as jeera in many Indian markets",
                "Use food-level spice only"
            ]),
            "Public plant photo shown for field recognition; use trusted kitchen cumin.",
        ),
        (
            25,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/Fenugreek_seeds.jpg/960px-Fenugreek_seeds.jpg",
            "https://commons.wikimedia.org/wiki/File:Fenugreek_seeds.jpg",
            json!([
                "Leaves and yellow-brown seeds used in food",
                "Known as methi in many Indian markets",
                "Avoid concentrated seed use in pregnancy"
            ]),
            "Public plant photo shown for field recognition; use trusted food-level fenugreek.",
        ),
        (
            26,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Allium_sativum%E5%A4%A7%E8%92%9C_%E8%92%9C%E5%A4%B4_Garlic.jpg/960px-Allium_sativum%E5%A4%A7%E8%92%9C_%E8%92%9C%E5%A4%B4_Garlic.jpg",
            "https://commons.wikimedia.org/wiki/File:Allium_sativum%E5%A4%A7%E8%92%9C_%E8%92%9C%E5%A4%B4_Garlic.jpg",
            json!([
                "White segmented bulb used in cooking",
                "Strong smell when crushed",
                "Do not apply raw garlic to skin"
            ]),
            "Public plant photo shown for field recognition; use normal cooked food garlic.",
        ),
        (
            27,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Alliumcepa.JPG/960px-Alliumcepa.JPG",
            "https://commons.wikimedia.org/wiki/File:Alliumcepa.JPG",
            json!([
                "Layered bulb vegetable used in cooking",
                "Known as pyaz in many Indian markets",
                "Can irritate reflux in some people"
            ]),
            "Public plant photo shown for field recognition; use normal cooked food onion.",
        ),
        (
            28,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/e/e1/Mint_leaves_%28Mentha_spicata%29.jpg/960px-Mint_leaves_%28Mentha_spicata%29.jpg",
            "https://commons.wikimedia.org/wiki/File:Mint_leaves_(Mentha_spicata).jpg",
            json!([
                "Aromatic mint leaf used in chutney or tea traditions",
                "Leaves smell minty when crushed",
                "Use leaf only, not essential oil dosing"
            ]),
            "Public plant photo shown for field recognition; use trusted market mint.",
        ),
        (
            29,
            "https://upload.wikimedia.org/wikipedia/commons/thumb/7/79/Murraya_koenigii_leaves.jpg/960px-Murraya_koenigii_leaves.jpg",
            "https://commons.wikimedia.org/wiki/File:Murraya_koenigii_leaves.jpg",
            json!([
                "Small glossy cooking leaves from a known curry leaf tree",
                "Usually used fresh in food",
                "Do not use unknown lookalike leaves"
            ]),
            "Public plant photo shown for field recognition; use known cooking leaves.",
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
        (27, 2, "India", "Bihar", 0.62, "demo market herb where available"),
        (28, 6, "India", "Bihar", 0.72, "demo common market mint"),
        (29, 7, "India", "Bihar", 0.58, "demo dried market herb where available"),
        (30, 8, "India", "Bihar", 0.78, "demo household garden plant"),
        (31, 9, "India", "Bihar", 0.8, "demo tropical fruit tree"),
        (32, 11, "India", "Bihar", 0.76, "demo cultivated leafy food"),
        (33, 14, "India", "Bihar", 0.75, "demo food crop option"),
        (34, 16, "India", "Bihar", 0.7, "demo cultivated household herb"),
        (35, 17, "India", "Bihar", 0.68, "demo familiar drink plant"),
        (36, 19, "India", "Bihar", 0.84, "demo common vegetable crop"),
        (37, 20, "India", "Bihar", 0.9, "demo common household tulsi plant"),
        (38, 21, "India", "Bihar", 0.82, "demo common food fruit"),
        (39, 22, "India", "Bihar", 0.9, "demo common kitchen herb"),
        (40, 23, "India", "Bihar", 0.88, "demo common market seed"),
        (41, 24, "India", "Bihar", 0.9, "demo common kitchen spice"),
        (42, 25, "India", "Bihar", 0.88, "demo common food herb"),
        (43, 26, "India", "Bihar", 0.92, "demo common kitchen bulb"),
        (44, 27, "India", "Bihar", 0.94, "demo common kitchen vegetable"),
        (45, 28, "India", "Bihar", 0.9, "demo common market herb"),
        (46, 29, "India", "Bihar", 0.86, "demo common cooking leaf"),
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
    retrieve_herbs_with_limit(pool, country, region, symptom, 5).await
}

pub async fn retrieve_herbs_with_limit(
    pool: &SqlitePool,
    country: Option<&str>,
    region: Option<&str>,
    symptom: Option<&str>,
    limit: usize,
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
    Ok(scored.into_iter().map(|(_, h)| h).take(limit).collect())
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
