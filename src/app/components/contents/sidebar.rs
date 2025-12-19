use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div { class: "sidebar",
            span {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur dignissim lorem quis elit mattis euismod. Integer felis ipsum, hendrerit sit amet nunc sit amet, maximus dictum enim. Phasellus est nisl, dapibus in pulvinar finibus, interdum at magna. Integer ipsum turpis, porta ut enim sed, lobortis euismod sem. Duis scelerisque lacus nec diam dignissim tempus. Mauris accumsan efficitur rhoncus. Nullam dapibus mollis ligula, nec hendrerit ligula tristique vel. Pellentesque vel mollis arcu, feugiat consectetur felis. Praesent at suscipit nulla.

Praesent consequat malesuada risus euismod suscipit. Nulla dignissim iaculis orci, vel sodales elit porta quis. Curabitur sit amet ullamcorper ante, vitae ultrices erat. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc ullamcorper neque eu dolor ullamcorper, ut gravida ipsum ornare. Morbi non elit at urna rutrum sodales sit amet non justo. Donec porttitor augue nec aliquet sollicitudin. Maecenas pharetra mi ut mollis consequat. Praesent vulputate egestas risus, eu tincidunt odio bibendum in. Aliquam mollis orci leo, nec scelerisque nisi porttitor ultricies.

Duis bibendum, lacus at egestas dapibus, enim ligula congue libero, a tempus tortor leo ut nibh. Donec in quam ut elit vulputate cursus. Donec luctus, felis ac tempus sollicitudin, est lorem ornare lectus, in venenatis metus est sed ex. Vivamus elementum lorem dolor, sit amet suscipit dolor tempus a. Maecenas lorem risus, pulvinar semper sollicitudin et, vestibulum vitae sapien. Nunc ante velit, sagittis sit amet venenatis sit amet, euismod quis nisi. Proin eget turpis a metus rhoncus faucibus et sed nibh. Pellentesque id rutrum odio, non aliquet erat. Sed lacinia leo vitae felis mollis malesuada. Suspendisse varius malesuada justo, et dapibus arcu semper vitae. Praesent dictum lobortis justo eu posuere. Vestibulum faucibus ante quam, ut dignissim purus porttitor id.

Duis eget elit condimentum, egestas arcu sed, tincidunt dui. In non ex quis dolor tristique feugiat. Ut euismod dapibus nisl sed condimentum. Nulla auctor consequat consectetur. Phasellus rhoncus dui ut elit maximus ornare. Fusce fringilla ante at cursus commodo. Nunc faucibus convallis lacinia. Etiam faucibus pulvinar porta.

Pellentesque rutrum dolor ligula, ac ornare libero aliquam quis. Vivamus viverra, velit a dictum suscipit, dolor dui dignissim leo, sed sagittis mauris lorem at felis. Integer in enim tempor ligula viverra feugiat eu eu ex. Vestibulum vehicula tempus metus ac aliquet. Proin placerat ullamcorper libero eget iaculis. Sed mollis ipsum et molestie vulputate. Nam ut nisi quis mauris rhoncus pretium. Proin tempor bibendum porttitor. Phasellus varius elit id neque sodales, in viverra leo tempor. Morbi sit amet ipsum feugiat, scelerisque orci ac, blandit elit. Praesent euismod nisi vel iaculis pellentesque. Integer sit amet dui vel sapien sodales fringilla. In hac habitasse platea dictumst. Phasellus posuere feugiat pharetra. Nulla eu nunc dignissim, elementum libero sit amet, viverra mauris. Proin egestas, orci eget suscipit iaculis, arcu tellus ultricies arcu, ac elementum diam mauris vel odio. "
            }
        }
    }
}
