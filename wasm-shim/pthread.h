#ifndef _BLOSC_PTHREAD_H
#define _BLOSC_PTHREAD_H	1

#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>

/* Types */
typedef int pthread_t;
typedef int pthread_attr_t;
typedef int pthread_mutex_t;
typedef int pthread_mutexattr_t;
typedef int pthread_cond_t;
typedef int pthread_condattr_t;
typedef int pthread_rwlock_t;
typedef int pthread_rwlockattr_t;
typedef int pthread_once_t;

/* Constants */
#define PTHREAD_CREATE_JOINABLE 0
#define PTHREAD_MUTEX_INITIALIZER 0
#define PTHREAD_COND_INITIALIZER 0
#define PTHREAD_RWLOCK_INITIALIZER 0
#define PTHREAD_ONCE_INIT 0

/* Function prototypes */

static inline int pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                                 void *(*start_routine)(void *), void *arg) {
    // In a single-threaded shim, we might choose to run the routine immediately
    // or just return an error. Running it immediately changes semantics significantly.
    // Often, libraries check for threading support. If this returns 0, they assume a thread started.
    // For simple shims, we often just return an error indicating threads aren't supported,
    // or if the code is structured to handle it, we might execute synchronously.
    // However, a common pattern for "no-op" shims is to pretend to succeed but do nothing,
    // which is dangerous if the code relies on side effects.
    // Given this is likely for Blosc in Wasm, we probably want to avoid actual threading.
    return 0; 
}

static inline int pthread_join(pthread_t thread, void **retval) {
    return 0;
}

static inline int pthread_detach(pthread_t thread) {
    return 0;
}

static inline int pthread_once(pthread_once_t *once_control, void (*init_routine)(void)) {
    if (*once_control == PTHREAD_ONCE_INIT) {
        init_routine();
        *once_control = 1;
    }
    return 0;
}

static inline int pthread_atfork(void (*prepare)(void), void (*parent)(void), void (*child)(void)) {
    return 0;
}

/* Mutex */
static inline int pthread_mutex_init(pthread_mutex_t *mutex,
                                     const pthread_mutexattr_t *attr) {
    return 0;
}

static inline int pthread_mutex_destroy(pthread_mutex_t *mutex) {
    return 0;
}

static inline int pthread_mutex_lock(pthread_mutex_t *mutex) {
    return 0;
}

static inline int pthread_mutex_trylock(pthread_mutex_t *mutex) {
    return 0;
}

static inline int pthread_mutex_unlock(pthread_mutex_t *mutex) {
    return 0;
}

/* Condition Variables */
static inline int pthread_cond_init(pthread_cond_t *cond,
                                    const pthread_condattr_t *attr) {
    return 0;
}

static inline int pthread_cond_destroy(pthread_cond_t *cond) {
    return 0;
}

static inline int pthread_cond_wait(pthread_cond_t *cond, pthread_mutex_t *mutex) {
    return 0;
}

static inline int pthread_cond_signal(pthread_cond_t *cond) {
    return 0;
}

static inline int pthread_cond_broadcast(pthread_cond_t *cond) {
    return 0;
}

/* Read-Write Locks */
static inline int pthread_rwlock_init(pthread_rwlock_t *rwlock,
                                      const pthread_rwlockattr_t *attr) {
    return 0;
}

static inline int pthread_rwlock_destroy(pthread_rwlock_t *rwlock) {
    return 0;
}

static inline int pthread_rwlock_rdlock(pthread_rwlock_t *rwlock) {
    return 0;
}

static inline int pthread_rwlock_wrlock(pthread_rwlock_t *rwlock) {
    return 0;
}

static inline int pthread_rwlock_unlock(pthread_rwlock_t *rwlock) {
    return 0;
}

/* Attributes */
static inline int pthread_attr_init(pthread_attr_t *attr) {
    return 0;
}

static inline int pthread_attr_destroy(pthread_attr_t *attr) {
    return 0;
}

static inline int pthread_attr_setdetachstate(pthread_attr_t *attr, int detachstate) {
    return 0;
}

#ifdef __cplusplus
}
#endif

#endif /* _BLOSC_PTHREAD_H */
