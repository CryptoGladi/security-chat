import toast from 'solid-toast';

export namespace Toast {
	enum Type {
		Success,
    Error
	}

	function base(text: string, type: Type) {
		toast.custom((t) => {
			return (
				<div class="alert h-auto w-auto" classList={{ 'alert-success': type === Type.Success, 'alert-error': type === Type.Error }}>
					<span>{text}</span>
				</div>
			);
		});
	}

	export function success(text: string) {
		base(text, Type.Success);
	}

  export function error(text: string) {
    base(text, Type.Error);
  }
}
