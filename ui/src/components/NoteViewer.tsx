'use client';

import { useEffect, useState } from 'react';
import { api, NoteDetail, ThreadData } from '@/lib/api';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import Link from 'next/link';

function ThreadChain({ thread }: { thread: ThreadData }) {
  return (
    <Card className="mt-8 p-4">
      <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">
        笔记链
      </p>
      <div className="space-y-1">
        {thread.notes.map((n, idx) => {
          const isCurrent = n.id === thread.notes[thread.notes.length - 1]?.id;
          return (
            <Link
              key={n.id}
              href={`/notes/${n.id}`}
              className={`block rounded-md px-2 py-1 text-sm transition-all duration-150 ${
                isCurrent
                  ? 'bg-accent font-medium text-foreground'
                  : 'text-muted-foreground hover:bg-accent hover:text-foreground'
              }`}
              style={{ paddingLeft: `${idx * 20 + 8}px` }}
            >
              <span className="mr-2 text-muted-foreground">└</span>
              <span className="text-xs text-muted-foreground mr-1">{n.category}</span>
              {n.title}
            </Link>
          );
        })}
      </div>
    </Card>
  );
}

export default function NoteViewer({ id }: { id: string }) {
  const [note, setNote] = useState<NoteDetail | null>(null);
  const [thread, setThread] = useState<ThreadData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    setLoading(true);
    setError('');
    Promise.all([
      api.getNote(id),
      api.getThread(id).catch(() => null),
    ])
      .then(([n, t]) => {
        setNote(n);
        setThread(t);
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, [id]);

  if (loading) {
    return (
      <div className="flex-1 overflow-y-auto p-8">
        <div className="max-w-3xl mx-auto space-y-4">
          <Skeleton className="h-8 w-3/4" />
          <Skeleton className="h-4 w-1/2" />
          <Skeleton className="h-4 w-full" />
          <Skeleton className="h-4 w-full" />
          <Skeleton className="h-4 w-2/3" />
        </div>
      </div>
    );
  }

  if (error || !note) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground text-sm">
        {error || '笔记不存在'}
      </div>
    );
  }

  const categoryColors: Record<string, string> = {
    issues: 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-300',
    articles: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300',
    ideas: 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-300',
    projects: 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300',
    journal: 'bg-pink-100 text-pink-700 dark:bg-pink-900/30 dark:text-pink-300',
  };

  return (
    <main className="flex-1 overflow-y-auto">
      <article className="max-w-3xl mx-auto px-12 py-10">
        {/* Meta */}
        <div className="mb-8">
          <div className="flex items-center gap-2 mb-4">
            <span className="text-3xl">{note.category === 'issues' ? '🔧' : note.category === 'articles' ? '📄' : note.category === 'ideas' ? '💡' : note.category === 'journal' ? '📓' : '📁'}</span>
            <Badge variant="outline" className="text-xs">
              {note.category}
            </Badge>
          </div>
          <h1 className="text-4xl font-semibold mb-4 text-foreground leading-tight">
            {note.title}
          </h1>
          <div className="flex items-center gap-2 flex-wrap">
            {note.tags.map((tag) => (
              <Badge key={tag} variant="secondary" className="text-xs">
                #{tag}
              </Badge>
            ))}
            <span className="text-xs text-muted-foreground ml-auto">
              {new Date(note.created_at).toLocaleString('zh-CN')}
            </span>
          </div>
        </div>

        {/* Content */}
        <div className="prose prose-sm dark:prose-invert max-w-none prose-headings:font-semibold prose-a:text-primary prose-code:bg-muted prose-code:px-1 prose-code:rounded prose-pre:bg-muted prose-pre:border prose-pre:border-border">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>
            {note.content || '*暂无内容*'}
          </ReactMarkdown>
        </div>

        {/* Thread */}
        {thread && thread.notes.length > 1 && <ThreadChain thread={thread} />}
      </article>
    </main>
  );
}
