# logging_spec_v1.md

## 1. Objetivo

Definir el modelo oficial de logging estructurado de la aplicación portable.

Este modelo garantiza:

* Trazabilidad completa de ejecuciones.
* Medición precisa de tiempos mediante `duration_ms`.
* Medición obligatoria de unidades de trabajo (`work_units`).
* Estructura extensible sin ruptura futura.
* Compatibilidad con migración a bases de datos.
* Separación clara entre metadata de ejecución y eventos secuenciales.

---

## 2. Estructura de Carpetas

Cada ejecución genera una carpeta:

```
user/runs/<tool_id>/<run_id>/
```

Dentro deben existir obligatoriamente:

```
run_metadata.json
execution_events.jsonl
```

Reglas:

* No se permite sobrescribir ejecuciones previas.
* No se permite eliminar runs.
* `execution_events.jsonl` es append-only.

---


## 3. Definición de run_id

El `run_id` identifica de forma única una ejecución (run).
Es una **cadena opaca**: ningún componente del sistema debe depender de parsearla para inferir fecha/hora o tool.

---

### 3.1 Formato Base Obligatorio

El formato base del `run_id` es:

```id="l7t8db"
RUN_YYYYMMDD_HHMMSS_mmm
```

Donde:

* `YYYYMMDD` → fecha (año, mes, día)
* `HHMMSS` → hora (hora, minuto, segundo)
* `mmm` → milisegundos (000–999)

Ejemplo:

```id="q8iz0d"
RUN_20260301_100000_100
```

---

### 3.2 Regla Anti-colisión (Obligatoria)

Si la carpeta `user/runs/<tool_id>/<run_id>/` ya existe, se aplicará un sufijo incremental para garantizar unicidad:

```id="2ne7lq"
RUN_YYYYMMDD_HHMMSS_mmm_01
RUN_YYYYMMDD_HHMMSS_mmm_02
...
```

Ejemplo:

```id="fyo3ff"
RUN_20260301_100000_100_01
```

Reglas:

* El sufijo incremental debe ser de 2 dígitos (`01`–`99`) como mínimo.
* Si se supera `99`, se permite continuar con más dígitos (`100`, `101`, …) si la implementación lo soporta.
* La generación del run_id es responsabilidad del entrypoint o del módulo de contexto de ejecución (`run_context`), no de las tools.

---

### 3.3 Reglas de Naming y Compatibilidad

* Solo se permiten caracteres:

  * letras mayúsculas A–Z
  * dígitos 0–9
  * guion bajo `_`
* No se permiten puntos `.` dentro del run_id.
* El run_id debe ser válido como nombre de carpeta en sistemas Windows/Linux.

---

### 3.4 Propiedades Garantizadas

El sistema garantiza:

* Unicidad dentro del árbol `user/runs/<tool_id>/`.
* Persistencia: un run_id no se reutiliza.
* Auditoría: todo run_id corresponde a un directorio con `run_metadata.json` y `execution_events.jsonl`.

---

### 3.5 Nota de diseño

Aunque el formato contiene información temporal, el run_id debe tratarse como identificador opaco.
El análisis temporal debe basarse en `started_at` / `timestamp`, no en el string del run_id.

---


## 4. run_metadata.json

`run_metadata.json` es el descriptor de ejecución (run) y actúa como cabecera estable del conjunto de logs.

Reglas estructurales:

* Debe existir exactamente un fichero `run_metadata.json` por run.
* Se crea al inicio de la ejecución.
* Solo se permite modificarlo **una vez** para completar los campos de cierre (fin de ejecución).
* No se permite eliminar runs ni sobrescribir runs existentes.

---

### 4.1 Estructura Obligatoria al Iniciar la Ejecución

Al inicio del run, el fichero debe contener como mínimo:

```json id="0v2cyy"
{
  "schema_version": "logging.run_metadata.v1",
  "run_id": "string",
  "tool_id": "string",
  "started_at": "ISO8601",
  "status": "running",
  "inputs": [],
  "work_units": {
    "primary": {
      "unit": "string",
      "count": 0
    }
  }
}
```

Definición de campos obligatorios:

* **schema_version**
  Obligatorio. Debe ser exactamente `"logging.run_metadata.v1"`.

* **run_id**
  Obligatorio. Identificador opaco del run (ver sección 3).

* **tool_id**
  Obligatorio. Identificador estable de la tool ejecutada.

* **started_at**
  Obligatorio. Timestamp ISO8601 con milisegundos.

* **status**
  Obligatorio.
  Valor permitido al inicio: `"running"`.

* **inputs**
  Obligatorio. Lista (puede estar vacía).
  Si se registran inputs, cada elemento debe ser un objeto JSON con, como mínimo:

  * `path` (string)
  * `content_hash` (string) cuando aplique

* **work_units**
  Obligatorio. Debe cumplir la estructura de la sección 6.
  Si todavía no se conoce la unidad real al inicio, se permite iniciar con una unidad neutral (p.ej. `files: 0`) y actualizarla al cierre.

---

### 4.2 Estructura Obligatoria al Finalizar la Ejecución

Al finalizar el run (status final distinto de `"running"`), el fichero debe incluir obligatoriamente:

```json id="w7e6qk"
{
  "ended_at": "ISO8601",
  "duration_ms": 0,
  "status": "completed | failed | aborted",
  "exit_code": 0
}
```

Reglas:

* **ended_at**
  Obligatorio. Timestamp ISO8601 con milisegundos.

* **duration_ms**
  Obligatorio. Entero ≥ 0. Medido con reloj monotónico.

* **status**
  Obligatorio. Valores permitidos al cierre:

  * `completed`
  * `failed`
  * `aborted`

* **exit_code**
  Obligatorio. Entero (convención: `0` éxito; no cero fallo/aborto).

* **work_units**
  Debe existir y reflejar la unidad real del run (ver sección 6).

---

### 4.3 Campos Opcionales Permitidos

Se permiten los siguientes campos opcionales (no obligatorios):

* tool_version
* runtime_version
* project_id
* config_snapshot
* environment
* summary
* warnings_count
* errors_count

Reglas:

* No se permiten nuevos campos fuera de esta lista sin incrementar versión mayor del schema.
* `warnings_count` y `errors_count` pueden derivarse de eventos, pero se admiten aquí como optimización.

---

### 4.4 Reglas de Consistencia

1. Si `status` es distinto de `"running"`, entonces deben existir:

   * `ended_at`
   * `duration_ms`
   * `exit_code`

2. Si el run está cerrado (`status` final), debe existir un evento `run_end` en `execution_events.jsonl`.

3. `duration_ms` debe ser coherente con la secuencia de eventos (no estrictamente igual, pero no negativa y razonable).

---

## 5. execution_events.jsonl

Formato: **JSON Lines (JSONL)**.
Cada línea contiene exactamente un objeto JSON válido y autónomo.

Reglas estructurales:

* Un objeto JSON por línea.
* No se permiten líneas en blanco.
* No se permite texto fuera de objetos JSON.
* El fichero es **append-only**.
* No se permite modificar ni eliminar eventos ya escritos.

---

### 5.1 Estructura Obligatoria de Cada Evento

Cada evento debe contener los siguientes campos obligatorios:

```json
{
  "schema_version": "logging.event.v1",
  "timestamp": "ISO8601",
  "level": "DEBUG | INFO | WARNING | ERROR | CRITICAL",
  "stage": "string",
  "event_type": "string",
  "message": "string"
}
```

Definición de campos:

* **schema_version**
  Obligatorio. Debe ser exactamente `"logging.event.v1"`.

* **timestamp**
  Obligatorio.
  Formato ISO8601 con milisegundos.
  Debe representar el instante de generación del evento.

* **level**
  Obligatorio.
  Debe pertenecer al catálogo de niveles definido en `logging_event_types_v1.json`.

* **stage**
  Obligatorio.
  Debe pertenecer al catálogo de stages definido en `logging_event_types_v1.json`.

* **event_type**
  Obligatorio.
  Debe pertenecer al catálogo de event_type definido en `logging_event_types_v1.json`.

* **message**
  Obligatorio.
  Cadena descriptiva breve.
  No debe contener datos sensibles ni contenido completo de usuario.

---

### 5.2 Campos Opcionales Permitidos

Los siguientes campos pueden añadirse cuando aplique:

* **data**
  Objeto JSON libre para metadatos adicionales.
  No debe contener:

  * Documentos completos
  * Prompts completos
  * Texto libre extenso del usuario

* **duration_ms**
  Entero ≥ 0.
  Obligatorio en eventos:

  * `stage_end`
  * `run_end`
    Representa la duración medida con reloj monotónico.

* **work_units**
  Estructura definida en la sección 6.
  Obligatorio en eventos:

  * `stage_end`
  * `run_end`

---

### 5.3 Reglas de Consistencia

1. Todo evento `stage_end` debe incluir:

   * `duration_ms`
   * `work_units`

2. Todo evento `run_end` debe incluir:

   * `duration_ms`
   * `work_units`

3. Si el `status` final en `run_metadata.json` es distinto de `"running"`, debe existir exactamente un evento `run_end`.

4. Los eventos deben registrarse en orden cronológico creciente.

---



## 6. work_units (Obligatorio)

El campo `work_units` representa el tamaño del trabajo procesado.
Es obligatorio para permitir análisis y estimación de tiempos (p.ej. `ms_por_pagina`, `ms_por_chunk`, etc.).

`work_units` es obligatorio en:

* `run_metadata.json` (siempre)
* eventos `stage_end` (siempre)
* evento `run_end` (siempre)

---

### 6.1 Estructura

```json id="1hr3uq"
"work_units": {
  "primary": {
    "unit": "string",
    "count": 0
  },
  "secondary": [
    {
      "unit": "string",
      "count": 0
    }
  ]
}
```

Definición:

* **primary** (obligatorio)
  Unidad principal del trabajo y su cantidad.

* **secondary** (opcional)
  Lista de unidades adicionales para describir el trabajo con más detalle.

---

### 6.2 Reglas de Validación

1. `primary` es obligatorio y debe contener:

   * `unit` (string no vacío)
   * `count` (entero >= 0)

2. Si existe `secondary`:

   * debe ser una lista
   * cada elemento debe tener `unit` y `count`
   * `count` debe ser entero >= 0

3. Las unidades (`unit`) deben pertenecer al catálogo definido en:

```id="fsgmyg"
work_units_catalog_v1.json
```

4. No se permiten unidades repetidas dentro de `secondary`.
   (Si se necesita agregar, se sumará el count y se registrará una única entrada por unidad).

---

### 6.3 Reglas de Interpretación

* `primary` debe representar la medida más representativa del coste del proceso en ese contexto:

  * OCR → `pages`
  * Embeddings → `chunks` (y opcionalmente `tokens` como secondary)
  * KG → `triples`
  * DoWhy → `rows`
  * Export → `files`

* `secondary` se utiliza para afinar análisis (ej. OCR con imágenes, embeddings con tokens, KG con entidades).

---

### 6.4 Reglas de Consistencia por Contexto

1. En eventos `stage_end`, `work_units` debe referirse al trabajo realizado **en esa etapa** (no global del run).

2. En el evento `run_end`, `work_units` debe referirse al trabajo procesado **a nivel de ejecución**.
   Se recomienda que use una unidad “global” coherente (p.ej. `files` o `documents`) aunque existan unidades más específicas en etapas internas.

3. En `run_metadata.json`, `work_units` debe reflejar la unidad global del run, y puede actualizarse al cierre si al inicio no estaba disponible.

---

### 6.5 Unidad Neutral Permitida (Inicio)

Si al iniciar el run todavía no se conoce el tamaño real del trabajo, se permite registrar una unidad neutral en `run_metadata.json`:

Ejemplo:

```json id="p17dvv"
"work_units": {
  "primary": {
    "unit": "files",
    "count": 0
  }
}
```

Regla:

* Esta unidad neutral debe actualizarse al finalizar la ejecución con el valor real.

---

### 6.6 Ejemplos

OCR:

```json id="snj3p0"
"work_units": {
  "primary": { "unit": "pages", "count": 42 },
  "secondary": [
    { "unit": "images", "count": 120 }
  ]
}
```

Embeddings:

```json id="o1cmw5"
"work_units": {
  "primary": { "unit": "chunks", "count": 350 },
  "secondary": [
    { "unit": "tokens", "count": 125000 },
    { "unit": "vectors", "count": 350 }
  ]
}
```

KG:

```json id="smyv6r"
"work_units": {
  "primary": { "unit": "triples", "count": 18000 },
  "secondary": [
    { "unit": "entities", "count": 2200 }
  ]
}
```

---


## 7. duration_ms

El campo `duration_ms` representa la duración medida de un proceso o etapa, expresada como entero en milisegundos.

`duration_ms` es obligatorio para permitir:

* análisis de rendimiento,
* comparación entre ejecuciones,
* estimación de tiempos por `work_units`.

---

### 7.1 Dónde es Obligatorio

`duration_ms` es obligatorio en:

1. `run_metadata.json` al cierre del run

   * campo: `duration_ms`

2. Evento `run_end` en `execution_events.jsonl`

   * campo: `duration_ms`

3. Eventos `stage_end` en `execution_events.jsonl`

   * campo: `duration_ms`

---

### 7.2 Reglas de Validación

* Debe ser un entero (`int`) >= 0.
* No se permiten valores negativos.
* No se permiten floats.
* No se permite omitirlo en `run_end` ni en `stage_end`.

---

### 7.3 Método de Medición (Obligatorio)

La medición de `duration_ms` debe realizarse utilizando un reloj **monotónico**, no el reloj del sistema.

Motivo:

* evita errores por cambios de hora,
* evita inconsistencias por sincronización NTP,
* mantiene estabilidad entre plataformas.

---

### 7.4 Reglas de Medición por Etapa

* `stage_start` puede registrarse sin `duration_ms`.
* `stage_end` debe registrar:

  * `duration_ms` como duración transcurrida desde el `stage_start` correspondiente,
  * `work_units` como tamaño del trabajo ejecutado en esa etapa.

Se recomienda (patrón estándar):

1. Emitir `stage_start`
2. Iniciar temporizador monotónico
3. Emitir `stage_end` con:

   * `duration_ms`
   * `work_units`

---

### 7.5 Reglas de Medición del Run Completo

El run completo debe medirse con temporizador monotónico desde el inicio hasta el cierre.

* `run_metadata.duration_ms` representa el tiempo total del run.
* `run_end.duration_ms` debe ser coherente con `run_metadata.duration_ms`.

Coherencia significa:

* ambos >= 0
* valores similares
* no necesariamente idénticos si existen pequeñas diferencias por orden de escritura, pero nunca divergencias absurdas.

---

### 7.6 Reglas de Interpretación con work_units

Para análisis posteriores se define la métrica derivada:

```id="7b9ag6"
ms_per_unit = duration_ms / work_units.primary.count
```

Reglas:

* Si `work_units.primary.count == 0`, no se calcula la métrica.
* Se recomienda registrar `work_units.primary.count` con valor real al cierre para permitir cálculo.

---

### 7.7 Ejemplos

OCR etapa:

```json id="9ho32r"
{
  "event_type": "stage_end",
  "duration_ms": 3300,
  "work_units": { "primary": { "unit": "pages", "count": 42 } }
}
```

Run completo:

```json id="1wqj8b"
{
  "status": "completed",
  "duration_ms": 8900,
  "work_units": { "primary": { "unit": "files", "count": 3 } }
}
```

---


## 8. Reglas de Escritura

* `execution_events.jsonl` es append-only.
* `run_metadata.json` solo puede modificarse para completar finalización.
* No se permite modificar eventos históricos.
* No se permite eliminar runs.

---


## 9. Privacidad y Seguridad

El logging está diseñado para registrar **métricas, trazabilidad y referencias**, no contenido sensible.

Objetivos:

* Evitar exposición accidental de información privada.
* Evitar crecimiento descontrolado del tamaño de logs.
* Permitir auditoría sin almacenar datos completos.

---

### 9.1 Principio General

Los logs deben contener:

* Identificadores (run_id, tool_id)
* Métricas (duration_ms, work_units)
* Estados (status, exit_code)
* Referencias (paths de artefactos, hashes)
* Mensajes breves y no sensibles

Los logs **no deben** contener:

* contenido completo de documentos
* texto libre extenso introducido por el usuario
* prompts completos de LLM
* respuestas completas de LLM
* claves, tokens, credenciales
* datos personales innecesarios

---

### 9.2 Prohibiciones Específicas

#### A) Documentos

Prohibido registrar en `data`:

* texto completo extraído de un PDF
* páginas OCR completas
* tablas completas
* dumps de archivos

✅ Permitido:

* `content_hash`
* número de páginas
* conteo de caracteres
* ruta del artefacto generado (p.ej. txt con OCR)

Ejemplo permitido:

```json id="sknqoy"
"data": {
  "content_hash": "sha256_abc123",
  "page_count": 42,
  "artifact_path": "user/outputs/ocr/MOD_BUQUE_X_2018_ocr.txt"
}
```

---

#### B) LLM (cuando exista)

Prohibido registrar:

* prompt completo
* respuesta completa
* contexto completo usado para inferencia
* contenido de mensajes de usuario

✅ Permitido:

* `llm_model_id`
* `prompt_hash`
* `response_hash`
* tokens usados
* artefacto referenciado (ruta) si se guarda en archivo aparte

Ejemplo permitido:

```json id="1e1eiq"
"data": {
  "llm_model_id": "mistral_7b_instruct_q4",
  "prompt_hash": "sha256_prompt_123",
  "response_hash": "sha256_resp_456",
  "tokens_in": 1200,
  "tokens_out": 350
}
```

---

#### C) Credenciales

Prohibido registrar cualquier tipo de:

* API keys
* tokens
* secretos
* rutas del sistema con credenciales embebidas

Si se detecta una credencial, debe:

* omitirse
* sustituirse por `"[REDACTED]"`

---

#### D) Texto libre del usuario

Prohibido registrar directamente texto libre que introduzca el usuario.

✅ Permitido:

* flags o códigos de razón (`reason_code`)
* contadores
* ids

Ejemplo permitido:

```json id="cv3r3g"
"data": {
  "reason_code": "missing_required_inputs",
  "missing_fields": ["sources", "template"]
}
```

---

### 9.3 Reglas de Redacción (Redaction)

Cuando sea necesario registrar un valor que podría contener contenido sensible:

* reemplazar por `"[REDACTED]"` o hash
* preferir hash antes que texto
* preferir path de artefacto antes que contenido embebido

---

### 9.4 Tamaño y Control de Ruido

* Los eventos deben ser concisos.
* `message` debe ser breve.
* El detalle técnico debe ir en `data`, pero solo como métricas o referencias.
* Evitar registrar eventos de alta frecuencia (scroll, mouse move, etc.).

---

### 9.5 Auditoría

Para auditoría y trazabilidad, se recomienda:

* Guardar artefactos (outputs) en archivos separados bajo `user/outputs/...`
* Registrar en logs solo:

  * hash
  * ruta
  * métricas básicas

Esto permite reconstrucción del proceso sin volcar contenido en logs.

---


## 10. Versionado y Compatibilidad

El sistema de logging se rige por versionado explícito de esquema.

El presente documento define:

```
logging_spec_v1
```

Y los siguientes identificadores de versión asociados:

* `logging.run_metadata.v1`
* `logging.event.v1`
* `logging.event_types.v1`
* `logging.work_units_catalog.v1`

---

### 10.1 Regla General

Toda modificación debe clasificarse como:

* Cambio compatible (no requiere versión mayor)
* Cambio incompatible (requiere versión mayor)

---

### 10.2 Cambios Compatibles (NO requieren v2)

Se consideran compatibles:

* Añadir nuevos `stage` al catálogo.
* Añadir nuevos `event_type`.
* Añadir nuevas unidades a `work_units_catalog`.
* Añadir nuevos campos opcionales en:

  * `run_metadata.json`
  * eventos (`execution_events.jsonl`)
* Añadir validaciones adicionales que no invaliden datos antiguos.

Estos cambios no deben alterar:

* Campos obligatorios existentes.
* Estructura base.
* Ubicación de archivos.

---

### 10.3 Cambios Incompatibles (Requieren v2)

Requieren incremento de versión mayor:

* Modificar o eliminar campos obligatorios.
* Cambiar estructura de `work_units`.
* Cambiar estructura base de carpetas.
* Cambiar nombre de archivos principales.
* Cambiar reglas fundamentales (ej. dejar de requerir `work_units`).
* Cambiar formato del `run_id`.

En estos casos se deberá crear:

* `logging.run_metadata.v2`
* `logging.event.v2`
* etc.

---

### 10.4 Regla de Compatibilidad hacia Atrás

El sistema debe poder:

* Leer runs generados con versiones anteriores.
* Validarlos según la versión indicada en `schema_version`.
* No asumir que todos los runs son v1.

El `validate_logs.py` debe:

* Detectar la versión.
* Aplicar las reglas correspondientes.
* No fallar automáticamente si detecta una versión antigua compatible.

---

### 10.5 Estrategia de Evolución

Si en el futuro se introduce `v2`:

1. No se deben modificar runs históricos.
2. Los nuevos runs usarán la nueva versión.
3. El sistema podrá convivir con múltiples versiones.
4. Se puede proporcionar herramienta opcional de migración si es necesario.

---

### 10.6 Principio de Estabilidad

El objetivo del versionado es:

* Evitar deuda técnica.
* Evitar reinterpretaciones ambiguas.
* Permitir auditoría futura.
* Garantizar que un run generado hoy pueda analizarse dentro de años sin ambigüedad.

---


## 11. Extensibilidad

El modelo está diseñado para soportar:

* OCR
* Embeddings
* Vector Search
* Knowledge Graph
* Análisis Causal
* LLM
* Métricas de rendimiento
* Eventos de UI y workflow

Sin modificar la estructura base.

---

## 12. Ejemplo Completo

### run_metadata.json

```json
{
  "schema_version": "logging.run_metadata.v1",
  "run_id": "RUN_20260301_100000_100",
  "tool_id": "tool_pdf_ocr_extract",
  "started_at": "2026-03-01T10:00:00.100",
  "ended_at": "2026-03-01T10:00:09.000",
  "duration_ms": 8900,
  "status": "completed",
  "exit_code": 0,
  "inputs": [],
  "work_units": {
    "primary": {
      "unit": "files",
      "count": 3
    }
  }
}
```

### execution_events.jsonl (fragmento)

```json
{"schema_version":"logging.event.v1","timestamp":"2026-03-01T10:00:00.100","level":"INFO","stage":"run","event_type":"run_start","message":"Execution started"}
{"schema_version":"logging.event.v1","timestamp":"2026-03-01T10:00:02.500","level":"INFO","stage":"ingest","event_type":"stage_end","message":"Ingestion completed","duration_ms":1250,"work_units":{"primary":{"unit":"files","count":3}}}
{"schema_version":"logging.event.v1","timestamp":"2026-03-01T10:00:05.900","level":"INFO","stage":"ocr","event_type":"stage_end","message":"OCR completed","duration_ms":3300,"work_units":{"primary":{"unit":"pages","count":42}}}
{"schema_version":"logging.event.v1","timestamp":"2026-03-01T10:00:09.000","level":"INFO","stage":"run","event_type":"run_end","message":"Execution finished","duration_ms":8900,"work_units":{"primary":{"unit":"files","count":3}}}
```

---

## Estado

Versión: logging_spec_v1
Estado: Estable para implementación

---
