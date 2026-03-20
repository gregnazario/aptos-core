/**
 * Simple in-memory cache with TTL support
 */

interface CacheEntry<T> {
  data: T;
  expires: number;
}

export class NetworkCache {
  private cache: Map<string, CacheEntry<any>>;
  private ttl: number; // Time to live in seconds

  constructor(ttl: number = 300) {
    this.cache = new Map();
    this.ttl = ttl;

    // Clean up expired entries every minute
    setInterval(() => this.cleanup(), 60000);
  }

  /**
   * Get value from cache if not expired
   */
  get<T>(key: string): T | null {
    const entry = this.cache.get(key);

    if (!entry) {
      return null;
    }

    if (Date.now() > entry.expires) {
      this.cache.delete(key);
      return null;
    }

    return entry.data as T;
  }

  /**
   * Set value in cache with TTL
   */
  set<T>(key: string, data: T, customTtl?: number): void {
    const ttl = customTtl || this.ttl;
    const expires = Date.now() + (ttl * 1000);

    this.cache.set(key, { data, expires });
  }

  /**
   * Delete entry from cache
   */
  delete(key: string): void {
    this.cache.delete(key);
  }

  /**
   * Clear all cache entries
   */
  clear(): void {
    this.cache.clear();
  }

  /**
   * Get cache statistics
   */
  stats(): { size: number; keys: string[] } {
    return {
      size: this.cache.size,
      keys: Array.from(this.cache.keys()),
    };
  }

  /**
   * Clean up expired entries
   */
  private cleanup(): void {
    const now = Date.now();
    let cleaned = 0;

    for (const [key, entry] of this.cache.entries()) {
      if (now > entry.expires) {
        this.cache.delete(key);
        cleaned++;
      }
    }

    if (cleaned > 0) {
      console.log(`Cache cleanup: removed ${cleaned} expired entries`);
    }
  }
}
