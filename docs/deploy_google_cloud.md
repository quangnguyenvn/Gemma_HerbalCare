# Deploy to Google Cloud Run

This project can run as a single Cloud Run service. The Rust/Axum backend serves both the API and the SvelteKit static frontend, so a single subdomain can host the whole app.

## Suggested Subdomain

Use:

```text
herbalcare.voidforge.pro
```

or:

```text
gemma.voidforge.pro
```

## Build and Deploy

From the repository root:

```bash
gcloud run deploy gemma-herbalcare \
  --source . \
  --region asia-northeast3 \
  --allow-unauthenticated \
  --set-env-vars GEMMA_PROVIDER=mock
```

If your preferred Google Cloud region is different, replace `asia-northeast3`.

The deployed service will expose:

```text
/
/about
/consult
/herbs
/api/consult
/api/herbs
/api/triage
/api/demo-cases
/health
```

## Map a Custom Subdomain

After the Cloud Run service is deployed:

```bash
gcloud beta run domain-mappings create \
  --service gemma-herbalcare \
  --domain herbalcare.voidforge.pro \
  --region asia-northeast3
```

Google Cloud will print the DNS records required for the domain mapping. Add those records wherever `voidforge.pro` DNS is managed.

Cloud Run custom domain mapping for fully managed Cloud Run currently uses the beta gcloud command and has limited availability in some regions. If the selected region does not support direct domain mapping, keep the Cloud Run service in that region and put Firebase Hosting or a Google Cloud external Application Load Balancer in front of it.

For a subdomain, this is usually a `CNAME` similar to:

```text
herbalcare  CNAME  ghs.googlehosted.com
```

Use the exact value printed by Google Cloud.

## Production Gemma Endpoint

The default deployment uses:

```text
GEMMA_PROVIDER=mock
```

To connect a Gemma-compatible HTTP endpoint:

```bash
gcloud run services update gemma-herbalcare \
  --region asia-northeast3 \
  --set-env-vars GEMMA_PROVIDER=http,GEMMA_MODEL=gemma4,GEMMA_API_URL=https://YOUR_ENDPOINT/api/generate
```

If the endpoint needs a key, store it in Secret Manager rather than committing it to the repo.

## Notes

- The app uses SQLite at `/tmp/gemma_herbalcare.db` in Cloud Run. This is fine for the hackathon demo because seed data is loaded on startup and consultation logs are ephemeral.
- For persistent logs in production, move consultation storage to Cloud SQL, Firestore, or another managed database.
- The frontend uses same-origin API calls by default, so no public backend URL is needed.
