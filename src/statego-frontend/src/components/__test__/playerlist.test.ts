import {test, expect} from "vitest";
import {mount} from "@vue/test-utils";

import PlayerList from "../PlayerList.vue";

test("mount component", async () => {
	expect(PlayerList).toBeTruthy();
	const wrapper = mount(PlayerList)
});
