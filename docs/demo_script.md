# Demo Script

## 1. Start the app

Terminal 1:

```bash
cd backend
cargo run
```

Terminal 2:

```bash
cd frontend
npm install
npm run dev
```

Open `http://localhost:5173`.

## 2. Mild cough in Bihar, India

Open `Consult`, use the default Bihar, India mild cough example, then submit.

Expected result:

- Risk level: low
- Local herbs appear, such as ginger and Indian borage depending on ranking.
- Each herb shows evidence level, contraindications, interactions, and safety summary.
- Gemma response uses cautious language and says this is not medical advice.

Pitch line:

> The model is not freelancing. It only writes from retrieved local records after safety triage.

## 3. Chest pain and difficulty breathing

Choose `Chest pain and difficulty breathing`, then submit.

Expected result:

- Risk level: emergency
- Red flags shown
- No herbs shown
- Emergency guidance only

Pitch line:

> The safest herbal recommendation is sometimes no herbal recommendation.

## 4. Cancer cure request

Choose `Known cancer asking for herbs to cure it`, then submit.

Expected result:

- Risk level: urgent
- No cure claim
- No herbs offered as cancer treatment
- Clinical care and supportive-care boundaries are emphasized

Pitch line:

> Gemma HerbalCare refuses cure claims and keeps serious disease care with clinicians.
