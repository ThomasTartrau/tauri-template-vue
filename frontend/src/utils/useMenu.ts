import { createSharedComposable } from '@vueuse/core'
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import type { Group } from '@/lib/menu'
import { Bookmark, LayoutGrid, Settings, SquarePen, Tag, Users } from 'lucide-vue-next'

function _useMenu() {
  const { currentRoute } = useRouter()
  const menuList = computed<Group[]>(() => {
    return [
      {
        groupLabel: '',
        menus: [
          {
            href: '/dashboard',
            label: 'Dashboard',
            active: currentRoute.value.fullPath.includes('/dashboard'),
            icon: LayoutGrid,
            submenus: [],
          },
        ],
      },
      {
        groupLabel: 'Contents',
        menus: [
          {
            href: '',
            label: 'Posts',
            active: currentRoute.value.fullPath.includes('/posts'),
            icon: SquarePen,
            submenus: [
              {
                href: '/posts',
                label: 'All Posts',
                active: currentRoute.value.fullPath === '/posts',
              },
              {
                href: '/posts/new',
                label: 'New Post',
                active: currentRoute.value.fullPath === '/posts/new',
              },
            ],
          },
          {
            href: '/categories',
            label: 'Categories',
            active: currentRoute.value.fullPath.includes('/categories'),
            icon: Bookmark,
            submenus: [],
          },
          {
            href: '/tags',
            label: 'Tags',
            active: currentRoute.value.fullPath.includes('/tags'),
            icon: Tag,
            submenus: [],
          },
        ],
      },
      {
        groupLabel: 'Settings',
        menus: [
          {
            href: '/users',
            label: 'Users',
            active: currentRoute.value.fullPath.includes('/users'),
            icon: Users,
            submenus: [],
          },
          {
            href: '/account',
            label: 'Account',
            active: currentRoute.value.fullPath.includes('/account'),
            icon: Settings,
            submenus: [],
          },
        ],
      },
    ]
  })

  return {
    menuList,
  }
}

export const useMenu = createSharedComposable(_useMenu)