import { useNavigate } from 'solid-start';
import { haveAccount } from '~/ts/api-tauri';

export default function Index() {
	const navigate = useNavigate();

	haveAccount().then((have_account) => {
		if (have_account) {
			navigate('/main');
		} else {
			navigate('/welcom');
		}
	});

	return <main></main>;
}
