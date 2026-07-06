'use client';

import { NoteSummary } from '@/lib/api';
import Link from 'next/link';

const CATEGORY_COLORS: Record<string, string> = {
  issues: 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-300',
  articles: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300',
  ideas: 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-300',
  projects: 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300',
  journal: 'bg-pink-100 text-pink-700 dark:bg-pink-900/30 dark:text-pink-300',
};

function timeAgo(dateStr: string): string {
  const ms = Date.now() - new Date(dateStr).getTime();
  const mins = Math.floor(ms / 60000);
  if (mins < 1) return '刚刚';
  if (mins < 60) return `${mins}分钟前`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `${hours}小时前`;
  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}天前`;
  return new Date(dateStr).toLocaleDateString('zh-CN');
}

export default function NoteList({
  notes,
  activeId,
}: {
  notes: NoteSummary[];
  activeId?: string;
}) {
  if (notes.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center text-gray-400 text-sm">
        暂无笔记
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto border-r border-gray-200 dark:border-gray-700">
      {notes.map((note) => (
        <Link
          key={note.id}
          href={`/notes/${note.id}`}
          className={`block px-4 py-3 border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors ${
            activeId === note.id ? 'bg-blue-50 dark:bg-blue-900/20 border-l-2 border-l-blue-500' : ''
          }`}
        >
          <div className="flex items-center gap-2 mb-1">
            <span className={`px-1.5 py-0.5 rounded text-[10px] font-medium ${CATEGORY_COLORS[note.category] || 'bg-gray-100 text-gray-600'}`}>
              {note.category}
            </span>
            {note.tags.slice(0, 3).map((tag) => (
              <span key={tag} className="text-[10px] text-gray-400 dark:text-gray-500">
                #{tag}
              </span>
            ))}
          </div>
          <p className="text-sm font-medium text-gray-900 dark:text-gray-100 truncate">
            {note.title}
          </p>
          <p className="text-xs text-gray-400 mt-0.5">{timeAgo(note.created_at)}</p>
        </Link>
      ))}
    </div>
  );
}
