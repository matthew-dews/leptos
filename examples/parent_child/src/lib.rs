use leptos::*;
use web_sys::MouseEvent;

// This highlights four different ways that child components can communicate
// with their parent:
// 1) <ButtonA/>: passing a WriteSignal as one of the child component props,
//    for the child component to write into and the parent to read
// 2) <ButtonB/>: passing a closure as one of the child component props, for
//    the child component to call
// 3) <ButtonC/>: adding an `on:` event listener to a component
// 4) <ButtonD/>: providing a context that is used in the component (rather than prop drilling)

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
pub fn App() -> impl IntoView {
    // just some signals to toggle three classes on our <p>
    let (red, set_red) = create_signal(false);
    let (right, set_right) = create_signal(false);
    let (italics, set_italics) = create_signal(false);
    let (smallcaps, set_smallcaps) = create_signal(false);

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `WriteSignal<bool>` contexts
    // and makes it easier to refer to it in ButtonC
    provide_context(SmallcapsContext(set_smallcaps));

    view! {

        <main>
            <p
                // class: attributes take F: Fn() => bool, and these signals all implement Fn()
                class:red=red
                class:right=right
                class:italics=italics
                class:smallcaps=smallcaps
            >
                "Lorem ipsum sit dolor amet."
            </p>

            // Button A: pass the signal setter
            <ButtonA setter=set_red/>

            // Button B: pass a closure
            <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>

            // Button C: use a regular event listener
            // setting an event listener on a component like this applies it
            // to each of the top-level elements the component returns
            <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

            // Button D gets its setter from context rather than props
            <ButtonD/>
        </main>
    }
}

/// Button A receives a signal setter and updates the signal itself
#[component]
pub fn ButtonA(
    /// Signal that will be toggled when the button is clicked.
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view! {

        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Red"
        </button>
    }
}

/// Button B receives a closure
#[component]
pub fn ButtonB<F>(
    /// Callback that will be invoked when the button is clicked.
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {

        <button
            on:click=on_click
        >
            "Toggle Right"
        </button>
    }

    // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent) + 'static
    // and save you from typing out the generic
    // the component macro actually expands to define a
    //
    // struct ButtonBProps<F> where F: Fn(MouseEvent) + 'static {
    //   on_click: F
    // }
    //
    // this is what allows us to have named props in our component invocation,
    // instead of an ordered list of function arguments
    // if Rust ever had named function arguments we could drop this requirement
}

/// Button C is a dummy: it renders a button but doesn't handle
/// its click. Instead, the parent component adds an event listener.
#[component]
pub fn ButtonC() -> impl IntoView {
    view! {

        <button>
            "Toggle Italics"
        </button>
    }
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop
/// we get it from the context
#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapsContext>().unwrap().0;

    view! {

        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Small Caps"
        </button>
    }
}
