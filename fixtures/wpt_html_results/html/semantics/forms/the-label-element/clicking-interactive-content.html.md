# html/semantics/forms/the-label-element/clicking-interactive-content.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-label-element/clicking-interactive-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Label event handling when a descendant interactive content is clicked</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<label id=label></label>
<template id=interactive-content>
  <a href="about:blank" onclick="event.preventDefault()"></a>
  <audio controls></audio>
  <button></button>
  <details></details>
  <embed>
  <iframe></iframe>
  <img usemap="">
  <input>
  <label>label</label>
  <select></select>
  <textarea></textarea>
  <video controls></video>
</template>

<script>
"use strict";

const interactiveContent = document.getElementById("interactive-content");
const interactiveElements = Array.from(interactiveContent.content.children);
const label = document.getElementById("label");

for (const srcInteractiveElement of interactiveElements) {
  test(t => {
    t.add_cleanup(() => {
      label.innerHTML = "";
    });

    const interactiveElement = srcInteractiveElement.cloneNode();
    label.appendChild(interactiveElement);

    let clicked = 0;
    interactiveElement.addEventListener("click", () => {
      clicked++;
    });
    interactiveElement.click();
    assert_equals(clicked, 1, "clicking interactive content");

    clicked = 0;
    const span = document.createElement("span");
    interactiveElement.appendChild(span);
    span.click();
    assert_equals(clicked, 1, "clicking descendant of interactive content");
  }, `interactive content ${srcInteractiveElement.outerHTML} as first child of <label>`);

  test(t => {
    t.add_cleanup(() => {
      label.innerHTML = "";
    });

    const interactiveElement = srcInteractiveElement.cloneNode();
    const div = document.createElement("div");
    div.appendChild(interactiveElement);
    label.appendChild(div);

    let clicked = 0;
    interactiveElement.addEventListener("click", () => {
      clicked++;
    });
    interactiveElement.click();
    assert_equals(clicked, 1, "clicking nested interactive content");

    clicked = 0;
    const span = document.createElement("span");
    interactiveElement.appendChild(span);
    span.click();
    assert_equals(clicked, 1, "clicking descendant of nested interactive content");
  }, `interactive content ${srcInteractiveElement.outerHTML} deeply nested under <label>`);

  test(t => {
    t.add_cleanup(() => {
      label.innerHTML = "";
    });

    const button = document.createElement("button");
    label.appendChild(button);

    const interactiveElement = srcInteractiveElement.cloneNode();
    label.appendChild(interactiveElement);

    let buttonClicked = 0;
    button.addEventListener("click", () => {
      buttonClicked++;
    });

    let clicked = 0;
    interactiveElement.addEventListener("click", () => {
      clicked++;
    });
    interactiveElement.click();
    assert_equals(clicked, 1, "clicking nested interactive content");
    assert_equals(buttonClicked, 0, "clicking nested interactive content should not click button");

    clicked = 0;
    const span = document.createElement("span");
    interactiveElement.appendChild(span);
    span.click();
    assert_equals(clicked, 1, "clicking descendant of nested interactive content");
    assert_equals(buttonClicked, 0, "clicking descendant of nested interactive content should not click button");
  }, `interactive content ${srcInteractiveElement.outerHTML} as second child under <label>`);
}

</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 426,
        "byte_start": 416,
        "col": 12,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 474,
        "byte_start": 459,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 474,
        "byte_start": 459,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 474,
        "byte_start": 459,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-label-element/clicking-interactive-content.html"
}
```
