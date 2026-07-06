'use client';

import { useState, useEffect, useCallback } from 'react';
import { api, TagCount, StatsData } from '@/lib/api';
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarInput,
  SidebarSeparator,
} from '@/components/ui/sidebar';
import { Badge } from '@/components/ui/badge';

const CATEGORIES = [
  { key: '', label: '全部笔记', icon: '📋' },
  { key: 'issues', label: 'Issues', icon: '🔧' },
  { key: 'articles', label: 'Articles', icon: '📄' },
  { key: 'ideas', label: 'Ideas', icon: '💡' },
  { key: 'projects', label: 'Projects', icon: '📁' },
  { key: 'journal', label: 'Journal', icon: '📓' },
];

export default function AppSidebar({
  onCategoryChange,
  onSearch,
  activeCategory,
}: {
  onCategoryChange: (cat: string) => void;
  onSearch: (q: string) => void;
  activeCategory: string;
}) {
  const [tags, setTags] = useState<TagCount[]>([]);
  const [query, setQuery] = useState('');

  useEffect(() => {
    api.listTags().then(setTags).catch(() => {});
  }, []);

  const handleSearch = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      onSearch(query);
    },
    [query, onSearch]
  );

  return (
    <Sidebar>
      <SidebarContent>
        {/* Search */}
        <div className="p-3">
          <form onSubmit={handleSearch}>
            <SidebarInput
              placeholder="搜索笔记... (Ctrl+K)"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Escape') { setQuery(''); onSearch(''); }
              }}
            />
          </form>
        </div>

        <SidebarSeparator />

        {/* Categories */}
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {CATEGORIES.map((cat) => (
                <SidebarMenuItem key={cat.key}>
                  <SidebarMenuButton
                    isActive={activeCategory === cat.key}
                    onClick={() => onCategoryChange(cat.key)}
                    tooltip={cat.label}
                  >
                    <span>{cat.icon}</span>
                    <span>{cat.label}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarSeparator />

        {/* Tags */}
        {tags.length > 0 && (
          <SidebarGroup>
            <SidebarGroupLabel>标签</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {tags.map((t) => (
                  <SidebarMenuItem key={t.tag}>
                    <SidebarMenuButton
                      onClick={() => { onCategoryChange(''); onSearch(t.tag); }}
                      size="sm"
                    >
                      <span className="text-muted-foreground">#</span>
                      <span>{t.tag}</span>
                      <Badge variant="secondary" className="ml-auto text-xs">
                        {t.count}
                      </Badge>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        )}
      </SidebarContent>
    </Sidebar>
  );
}
