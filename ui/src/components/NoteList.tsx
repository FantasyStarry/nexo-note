'use client';

import { NoteSummary } from '@/lib/api';
import Link from 'next/link';
import { Badge } from '@/components/ui/badge';
import { Card } from '@/components/ui/card';

const CATEGORY_COLORS: Record<string, string> = {
  issues: 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-300',
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
      <div className="flex-1 flex items-center justify-center text-muted-foreground text-sm">
        暂无笔记
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto p-2">
      {notes.map((note) => (
        <Link
          key={note.id}
          href={`/notes/${note.id}`}
          className={`block mb-2 rounded-lg border p-3 transition-all duration-200 hover:shadow-md hover:-translate-y-0.5 ${
            activeId === note.id
              ? 'border-l-4 border-primary/60 bg-blue-50/80 shadow-sm'
              : 'border-l border-border/50 bg-card hover:border-border/80'
          }`}
          style={{ borderLeftColor: activeId === note.id ? 'var(--color-primary-500)' : undefined }}
        >
          {/* Category & Tags */}
          <div className="flex items-center gap-2 mb-2">
            <Badge
              variant="secondary"
              className={`text-[10px] px-1.5 py-0.5 font-medium ${CATEGORY_COLORS[note.category] || 'bg-gray-100 text-gray-600'}`}
            >
              {note.category}
            </Badge>
            {note.tags.slice(0, 3).map((tag) => (
              <span key={tag} className="text-[10px] text-muted-foreground">
                #{tag}
              </span>
            ))}
          </div>

          {/* Title */}
          <h3 className={`text-sm font-semibold mb-1 line-clamp-2 ${activeId === note.id ? 'text-primary' : 'text-foreground/90'}`}>
            {note.title}
          </h3>

          {/* Time */}
          <p className="text-xs text-muted-foreground">
            {timeAgo(note.created_at)}
          </p>
        </Link>
      ))}
    </div>
  );
}
