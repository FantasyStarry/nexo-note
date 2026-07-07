"use client";

import { useEffect, useState } from "react";
import { api, NoteDetail, ThreadData } from "@/lib/api";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import Link from "next/link";
import { CATEGORY_BADGE, CATEGORY_META, categoryLabel } from "@/lib/categories";
import { cn } from "@/lib/utils";

function ThreadChain({ thread }: { thread: ThreadData }) {
  return (
    <Card className="mt-10 border-border/70 bg-transparent p-4 ring-0">
      <p className="mb-3 text-[11px] font-medium uppercase tracking-wider text-tertiary">
        笔记链
      </p>
      <div className="space-y-0.5">
        {thread.notes.map((n, idx) => {
          const isCurrent =
            n.id === thread.notes[thread.notes.length - 1]?.id;
          return (
            <Link
              key={n.id}
              href={`/notes/${n.id}`}
              style={{ paddingLeft: `${idx * 18 + 8}px` }}
              className={cn(
                "block rounded-md py-1 pr-2 text-[13px] transition-colors duration-150",
                isCurrent
                  ? "font-medium text-foreground"
                  : "text-muted-foreground hover:bg-[rgba(55,53,47,0.04)] hover:text-foreground"
              )}
            >
              <span className="mr-2 text-muted-foreground/70">└</span>
              <span className="mr-1.5 text-[11px] text-muted-foreground">
                {n.category}
              </span>
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
  const [error, setError] = useState("");

  useEffect(() => {
    setLoading(true);
    setError("");
    Promise.all([api.getNote(id), api.getThread(id).catch(() => null)])
      .then(([n, t]) => {
        setNote(n);
        setThread(t);
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, [id]);

  if (loading) {
    return (
      <div className="flex-1 overflow-y-auto px-6 py-12 md:px-16">
        <div className="mx-auto max-w-3xl space-y-4">
          <Skeleton className="h-9 w-3/4" />
          <Skeleton className="h-4 w-1/2" />
          <div className="h-6" />
          <Skeleton className="h-4 w-full" />
          <Skeleton className="h-4 w-full" />
          <Skeleton className="h-4 w-2/3" />
        </div>
      </div>
    );
  }

  if (error || !note) {
    return (
      <div className="flex-1 flex items-center justify-center text-[13px] text-muted-foreground">
        {error || "笔记不存在"}
      </div>
    );
  }

  const Icon = CATEGORY_META[note.category]?.icon;

  return (
    <main className="flex-1 overflow-y-auto">
      <article className="mx-auto max-w-3xl px-6 py-12 md:px-16 md:py-16">
        {/* Meta */}
        <div className="mb-8">
          <div className="mb-3 flex items-center gap-2">
            {Icon && <Icon className="size-5 text-muted-foreground" />}
            <span
              className={cn(
                "rounded px-1.5 py-0.5 text-[12px] font-medium",
                CATEGORY_BADGE[note.category] || "bg-muted text-muted-foreground"
              )}
            >
              {categoryLabel(note.category)}
            </span>
          </div>
          <h1 className="mb-4 text-[2.25rem] font-bold leading-tight tracking-tight text-foreground">
            {note.title}
          </h1>
          <div className="flex flex-wrap items-center gap-2 text-[13px] text-muted-foreground">
            {note.tags.map((tag) => (
              <span
                key={tag}
                className="rounded bg-muted px-2 py-0.5 text-muted-foreground"
              >
                #{tag}
              </span>
            ))}
            <span className="ml-auto tabular-nums">
              {new Date(note.created_at).toLocaleString("zh-CN")}
            </span>
          </div>
        </div>

        {/* Content */}
        <div className="prose prose-neutral max-w-none prose-headings:font-semibold prose-headings:tracking-tight prose-p:leading-[1.75] prose-a:text-[#2383e2] prose-a:no-underline hover:prose-a:underline prose-code:rounded prose-code:bg-muted prose-code:px-1 prose-code:py-0.5 prose-code:text-[0.85em] prose-code:before:content-none prose-code:after:content-none prose-pre:border prose-pre:border-border prose-pre:bg-[#f7f6f3] prose-pre:text-foreground prose-pre:rounded-lg dark:prose-invert dark:prose-pre:bg-white/[0.04]">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>
            {note.content || "_暂无内容_"}
          </ReactMarkdown>
        </div>

        {/* Thread */}
        {thread && thread.notes.length > 1 && <ThreadChain thread={thread} />}
      </article>
    </main>
  );
}
