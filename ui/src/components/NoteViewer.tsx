"use client";

import { useEffect, useState } from "react";
import { api, NoteDetail } from "@/lib/api";
import { Skeleton } from "@/components/ui/skeleton";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import Link from "next/link";
import { GitBranch } from "lucide-react";
import { CATEGORY_META, categoryLabel } from "@/lib/categories";

export default function NoteViewer({ id }: { id: string }) {
  const [note, setNote] = useState<NoteDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    setLoading(true);
    setError("");
    api.getNote(id)
      .then(setNote)
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, [id]);

  if (loading && !note) {
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

  if (!note) {
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
          <div className="mb-3 flex items-center gap-2 text-[13px] text-muted-foreground">
            {Icon && <Icon className="size-4" />}
            <span>{categoryLabel(note.category)}</span>
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
                {tag}
              </span>
            ))}
            <span className="tabular-nums ml-auto">
              {new Date(note.created_at).toLocaleString("zh-CN")}
            </span>
          </div>
          <Link
            href={`/thread/${id}${typeof window !== "undefined" && window.location.search ? window.location.search : ""}`}
            className="mt-4 inline-flex w-fit items-center gap-1.5 rounded-md border border-[#0d7d72]/25 px-3 py-1.5 text-[13px] text-[#0d7d72] transition-colors hover:bg-[rgba(13,125,114,0.06)] hover:text-[#0a655c]"
          >
            <GitBranch className="size-3.5" />
            查看笔记链
          </Link>
        </div>

        {/* Content */}
        <div className="prose prose-neutral max-w-none prose-headings:font-semibold prose-headings:tracking-tight prose-p:leading-[1.75] prose-a:text-[#0d7d72] prose-a:no-underline hover:prose-a:underline prose-code:rounded prose-code:bg-muted prose-code:px-1 prose-code:py-0.5 prose-code:text-[0.85em] prose-code:before:content-none prose-code:after:content-none prose-pre:border prose-pre:border-border prose-pre:bg-[#f7f6f3] prose-pre:text-foreground prose-pre:rounded-lg dark:prose-invert dark:prose-pre:bg-white/[0.04]">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>
            {note.content || "_暂无内容_"}
          </ReactMarkdown>
        </div>
      </article>
    </main>
  );
}
