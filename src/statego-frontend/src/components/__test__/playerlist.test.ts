import {test, expect} from "vitest";
import {mount} from "@vue/test-utils";

import PlayerList from "../PlayerList.vue";

test("mount component", async () => {
	expect(PlayerList).toBeTruthy();
	const wrapper = mount(PlayerList, {
		props: {
			winner: "Player 1",
			players: ['Player 1', 'Player 2', 'Player 3', 'Player 4', 'Player 5']
		}
	});
	expect(wrapper.text()).toContain("Player 1");
	expect(wrapper.text()).toContain("Player 2");
	expect(wrapper.text()).toContain("Player 3");
	expect(wrapper.text()).toContain("Player 4");
	expect(wrapper.text()).toContain("Player 5");
	
});
