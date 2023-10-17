import { test, expect } from 'vitest';
import { mount } from '@vue/test-utils';

import SessionNote from '../SessionNote.vue';

test('mount component', async () => {
   expect(SessionNote).toBeTruthy();
   const wrapper = mount(SessionNote, {
      props: {
         notes: 'this is a note for the current session',
      },
   });
   expect(wrapper.text()).toContain('this is a note for the current session');
});
