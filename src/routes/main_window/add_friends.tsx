import { SideBar } from '~/components/side_bar';
import { Tabs, Tab } from '~/components/tabs';
import { AllFriends } from '~/components/add_friends/all_friends';
import { RequestsFriends } from '~/components/add_friends/requests_friends';
import { AddFriends } from '~/components/add_friends/add_friends';

const ConstTabs: Tab[] = [
	new Tab('Ваши друзья', AllFriends),
	new Tab('Запросы', AddFriends),
	new Tab('Ваши Заявки', RequestsFriends)
];

export default function Index() {
	return (
		<main class="flex">
			<SideBar />
			<Tabs tabs={ConstTabs} default_index={0} />
		</main>
	);
}
