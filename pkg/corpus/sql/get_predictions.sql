-- contrast all predictions on a statement
SELECT
    lower(hex(stmt.id)) AS statement_id
  , lower(hex(fingerprint.fingerprint)) AS fingerprint
  , prediction.valid
  , oracle.name AS oracle_name
  , stmt.text AS statement_text
  , lang.name AS language_name
  , prediction.message
  , urls.url
  , urls.license_id -- this can be further joined on licenses.id
FROM predictions                  AS prediction
INNER JOIN statements             AS stmt        ON  prediction.statement_id = stmt.id
LEFT  JOIN oracles                AS oracle      ON     prediction.oracle_id = oracle.id
LEFT  JOIN languages              AS lang        ON   prediction.language_id = lang.id
LEFT  JOIN statement_fingerprints AS fingerprint ON                  stmt.id = fingerprint.statement_id
LEFT  JOIN document_statements    AS src         ON                  stmt.id = src.statement_id
LEFT  JOIN document_urls          AS doc_url     ON          src.document_id = doc_url.document_id
LEFT  JOIN urls                                  ON doc_url.url_id = urls.id
;