'use client';

import { useCallback, useEffect, useState } from 'react';
import { api, NoteSummary, TagCount, StatsData } from '@/lib/api';
import Link from 'next/link';

const CATEGORIES = [
  { key: '', label: '全部', icon: '📋' },
  { key: 'issues', label: 'issues', icon: '📋' },
  { key: 'articles', label: 'articles', icon: '📄' },
  { key: 'ideas', label: 'ideas', icon: '💡' },
  { key: 'projects', label: 'projects', icon: '📁' },
  { key: 'journal', label: 'journal', icon: '📓' },
];

export default function Sidebar({
  onCategoryChange,
  onSearch,
  activeCategory,
  activeNoteId,
}: {
  onCategoryChange: (cat: string) => void;
  onSearch: (q: string) => void;
  activeCategory: string;
  activeNoteId?: string;
}) {
  const [tags, setTags] = useState<TagCount[]>([]);
  const [stats, setStats] = useState<StatsData | null>(null);
  const [query, setQuery] = useState('');

  useEffect(() => {
    api.listTags().then(setTags).catch(() => {});
    api.getStats().then(setStats).catch(() => {});
  }, []);

  const handleSearch = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      onSearch(query);
    },
    [query, onSearch]
  );

  return (
    <aside className="w-56 border-r border-gray-200 dark:border-gray-700 flex flex-col bg-gray-50 dark:bg-gray-900 h-full overflow-hidden">
      {/* Search */}
      <div className="p-3">
        <form onSubmit={handleSearch}>
          <input
            className="w-full px-3 py-1.5 text-sm rounded-md border border-gray-200 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-blue-500"
            placeholder="搜索笔记... (Ctrl+K)"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Escape') { setQuery(''); onSearch(''); }
            }}
          />
        </form>
      </div>

      {/* Categories */}
      <nav className="flex-1 overflow-y-auto px-2">
        {CATEGORIES.map((cat) => (
          <button
            key={cat.key}
            onClick={() => onCategoryChange(cat.key)}
            className={`w-full text-left px-3 py-1.5 rounded-md text-sm mb-0.5 flex items-center gap-2 transition-colors ${
              activeCategory === cat.key
                ? 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300 font-medium'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
            }`}
          >
            <span>{cat.icon}</span>
            <span>{cat.label}</span>
            {stats && (
              <span className="ml-auto text-xs text-gray-400">
                {stats.total_notes}
              </span>
            )}
          </button>
        ))}

        {/* Tags */}
        {tags.length > 0 && (
          <>
            <div className="mt-4 mb-1 px-3 text-xs text-gray-400 uppercase tracking-wider">
              标签
            </div>
            {tags.map((t) => (
              <button
                key={t.tag}
                onClick={() => { onCategoryChange(''); onSearch(t.tag); }}
                className="w-full text-left px-3 py-1 rounded-md text-xs text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 flex items-center gap-1"
              >
                <span>#</span>
                <span>{t.tag}</span>
                <span className="ml-auto text-gray-400">{t.count}</span>
              </button>
            ))}
          </>
        )}
      </nav>
    </aside>
  );
}
