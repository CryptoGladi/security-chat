import { useNavigate } from 'solid-start';
import { have_account } from '~/ts/api-tauri';

export default function Index() {
	const navigate = useNavigate();

	have_account().then((have_account) => {
		if (have_account) {
			navigate('/main');
		} else {
			navigate('/welcom');
		}
	});

	return <main></main>;
}
