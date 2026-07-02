#pragma once
#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

/** Opaque handle to a password character pool. */
typedef struct DeadpoolPool DeadpoolPool;

/* ── Lifecycle ──────────────────────────────────────────────────────────────── */

/** Allocates a new empty pool. Must be freed with deadpool_pool_free(). */
DeadpoolPool* deadpool_pool_new(void);

/** Frees a pool. Safe to call with NULL. */
void deadpool_pool_free(DeadpoolPool* pool);

/** Returns 1 if the pool contains no characters, 0 otherwise. */
int deadpool_pool_is_empty(const DeadpoolPool* pool);

/* ── Character sets ─────────────────────────────────────────────────────────── */

void deadpool_extend_lowercase   (DeadpoolPool* pool);
void deadpool_extend_uppercase   (DeadpoolPool* pool);
void deadpool_extend_digits      (DeadpoolPool* pool);
void deadpool_extend_braces      (DeadpoolPool* pool);
void deadpool_extend_punctuation (DeadpoolPool* pool);
void deadpool_extend_quotes      (DeadpoolPool* pool);
void deadpool_extend_dashes      (DeadpoolPool* pool);
void deadpool_extend_math        (DeadpoolPool* pool);
void deadpool_extend_logograms   (DeadpoolPool* pool);

/* ── Include / Exclude ──────────────────────────────────────────────────────── */

/**
 * Adds individual characters from a NUL-terminated string to the pool.
 * Returns 0 on success, -1 on error.
 */
int  deadpool_extend_from_string (DeadpoolPool* pool, const char* chars);

/** Marks characters as excluded from generation output. */
void deadpool_exclude_chars      (DeadpoolPool* pool, const char* chars);

/* ── Generation ─────────────────────────────────────────────────────────────── */

/**
 * Generates a password of `length` characters into `out_buf` (caller-allocated,
 * `out_len` bytes). Returns 0 on success, -1 on error.
 * Call deadpool_last_error() to retrieve the error message.
 */
int deadpool_generate(const DeadpoolPool* pool,
                      size_t length,
                      char* out_buf,
                      size_t out_len);

/* ── Diagnostics ────────────────────────────────────────────────────────────── */

/** Returns the last error message for this thread, or NULL if no error. */
const char* deadpool_last_error(void);

/** Returns a static NUL-terminated version string (e.g. "1.6.7"). */
const char* deadpool_version(void);

#ifdef __cplusplus
}
#endif
