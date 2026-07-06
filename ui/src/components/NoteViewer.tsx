'use client';

import { useEffect, useState } from 'react';
import { api, NoteDetail, ThreadData } from '@/lib/api';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import Link from 'next/link';

function ThreadChain({ thread }: { thread: ThreadData }) {
  return (
    <div className="mt-8 pt-4 border-t border-gray-200 dark:border-gray-700">
      <p className="text-[10px] uppercase tracking-wider text-gray-400 mb-2">笔记链</p>
      <div className="space-y-0.5 text-sm">
        {thread.notes.map((n) => {
          const depth = thread.notes.findIndex((x) => x.id === n.id);
          const isCurrent = n.id === thread.notes[thread.notes.length - 1]?.id;
          return (
            <Link
              key={n.id}
              href={`/notes/${n.id}`}
              className={`block hover:bg-gray-50 dark:hover:bg-gray-800 rounded px-2 py-0.5 ${
                isCurrent ? 'text-gray-900 dark:text-gray-100 font-medium' : 'text-gray-500 dark:text-gray-400'
              }`}
              style={{ paddingLeft: `${depth * 20 + 8}px` }}
            >
              <span className="mr-2 text-xs text-gray-300">{'└'}</span>
              <span className="text-[11px] text-gray-400 mr-1">{n.category}</span>
              {n.title}
            </Link>
          );
        })}
      </div>
    </div>
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
      <div className="flex-1 flex items-center justify-center">
        <div className="animate-spin w-6 h-6 border-2 border-blue-500 border-t-transparent rounded-full" />
      </div>
    );
  }

  if (error || !note) {
    return (
      <div className="flex-1 flex items-center justify-center text-gray-400 text-sm">
        {error || '笔记不存在'}
      </div>
    );
  }

  return (
    <main className="flex-1 overflow-y-auto">
      <article className="max-w-3xl mx-auto px-12 py-10">
        {/* Meta */}
        <div className="mb-6">
          <div className="flex items-center gap-2 mb-3">
            <span className="text-2xl">{note.category === 'issues' ? '📋' : note.category === 'articles' ? '📄' : note.category === 'ideas' ? '💡' : note.category === 'journal' ? '📓' : '📁'}</span>
            <span className="text-xs text-gray-400">{note.id}</span>
          </div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            {note.title}
          </h1>
          <div className="flex items-center gap-2 flex-wrap">
            {note.tags.map((tag) => (
              <span key={tag} className="px-2 py-0.5 text-xs rounded-full bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-300">
                {tag}
              </span>
            ))}
            <span className="text-xs text-gray-400 ml-auto">
              {new Date(note.created_at).toLocaleString('zh-CN')}
            </span>
          </div>
        </div>

        {/* Content */}
        <div className="prose prose-sm dark:prose-invert max-w-none prose-headings:font-semibold prose-a:text-blue-600 prose-code:bg-gray-100 dark:prose-code:bg-gray-800 prose-code:px-1 prose-code:rounded prose-pre:bg-gray-100 dark:prose-pre:bg-gray-800 prose-pre:border prose-pre:border-gray-200 dark:prose-pre:border-gray-700">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
          >
            {note.content || '*暂无内容*'}
          </ReactMarkdown>
        </div>

        {/* Thread */}
        {thread && thread.notes.length > 1 && <ThreadChain thread={thread} />}
      </article>
    </main>
  );
}
