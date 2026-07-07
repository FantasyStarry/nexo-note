"use client";

import Link from "next/link";
import { ThreadNote } from "@/lib/api";
import { CATEGORY_META } from "@/lib/categories";
import { cn } from "@/lib/utils";

function timeAgo(dateStr: string): string {
  const ms = Date.now() - new Date(dateStr).getTime();
  const mins = Math.floor(ms / 60000);
  if (mins < 1) return "刚刚";
  if (mins < 60) return `${mins}分钟前`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `${hours}小时前`;
  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}天前`;
  return new Date(dateStr).toLocaleDateString("zh-CN");
}

export default function ThreadGraph({
  notes,
  currentId,
}: {
  notes: ThreadNote[];
  currentId: string;
}) {
  return (
    <div className="flex flex-col items-center gap-0 py-8">
      {notes.map((n, idx) => {
        const isCurrent = n.id === currentId;
        const Icon = CATEGORY_META[n.category]?.icon;
        return (
          <div key={n.id} className="flex flex-col items-center">
            <Link
              href={`/thread/${n.id}`}
              className={cn(
                "flex w-80 max-w-[calc(100vw-3rem)] items-start gap-3 rounded-xl border px-4 py-3 transition-colors",
                isCurrent
                  ? "border-border bg-background shadow-sm"
                  : "border-border/60 bg-transparent hover:bg-[rgba(55,53,47,0.03)]"
              )}
            >
              {Icon && (
                <Icon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
              )}
              <div className="min-w-0">
                <p className="truncate text-[14px] font-medium">{n.title}</p>
                <p className="text-[12px] text-muted-foreground">
                  {n.category} · {timeAgo(n.created_at)}
                </p>
              </div>
            </Link>
            {idx < notes.length - 1 && <div className="h-8 w-px bg-border" />}
          </div>
        );
      })}
    </div>
  );
}
