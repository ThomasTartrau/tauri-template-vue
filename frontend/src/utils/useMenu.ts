import { createSharedComposable } from '@vueuse/core'
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { LayoutGrid, Settings, Users } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'
import type { Group } from '@/lib/menu'

function _useMenu() {
  const { currentRoute } = useRouter()
  const { t } = useI18n({ useScope: 'global' })
  const menuList = computed<Group[]>(() => {
    return [
      {
        groupLabel: '',
        menus: [
          {
            href: '/dashboard',
            label: t('sidebar.dashboard_label'),
            active: currentRoute.value.fullPath.includes('/dashboard'),
            icon: LayoutGrid,
            submenus: [],
          },
        ],
      },
      /* {
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
      }, */
      {
        groupLabel: t('sidebar.settings_group_label.label'),
        menus: [
          {
            href: '/users',
            label: t('sidebar.settings_group_label.menus.personnal_label'),
            active: currentRoute.value.fullPath.includes('/users'),
            icon: Users,
            submenus: [],
          },
          {
            href: '/security',
            label: t('sidebar.settings_group_label.menus.security_label'),
            active: currentRoute.value.fullPath.includes('/security'),
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
