import {
  FileText,
  Bug,
  Lightbulb,
  FolderKanban,
  BookOpen,
  LayoutList,
  type LucideIcon,
} from "lucide-react";

export const CATEGORY_META: Record<
  string,
  { label: string; icon: LucideIcon }
> = {
  "": { label: "全部笔记", icon: LayoutList },
  issues: { label: "问题", icon: Bug },
  articles: { label: "文章", icon: FileText },
  ideas: { label: "想法", icon: Lightbulb },
  projects: { label: "项目", icon: FolderKanban },
  journal: { label: "日志", icon: BookOpen },
};

// Soft, muted tag palette in the Notion spirit (pastel tint + deeper text).
export const CATEGORY_BADGE: Record<string, string> = {
  issues: "bg-[#fbe9d6] text-[#b4511b]",
  articles: "bg-[#e3eef6] text-[#0b6e99]",
  ideas: "bg-[#efe8f6] text-[#6940a5]",
  projects: "bg-[#dcefe6] text-[#0f7b46]",
  journal: "bg-[#f8e3f0] text-[#ad1a72]",
};

export function categoryLabel(key: string): string {
  return CATEGORY_META[key]?.label ?? key;
}
