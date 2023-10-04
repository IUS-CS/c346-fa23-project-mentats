import { test, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import SignupForm from '../SignupForm.vue'

test('mount component', async () => {
  expect(SignupForm).toBeTruthy()
  const wrapper = mount(SignupForm, {})
  expect(wrapper.text()).toContain('Sign Up')
})
