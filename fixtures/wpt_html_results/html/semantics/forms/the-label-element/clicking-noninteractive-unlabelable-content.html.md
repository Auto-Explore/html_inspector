# html/semantics/forms/the-label-element/clicking-noninteractive-unlabelable-content.html

Counts:
- errors: 1
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-label-element/clicking-noninteractive-unlabelable-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Label event handling when a descendant noninteractive and unlabelable content is clicked</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<label id=label></label>
<template id=noninteractive-unlabelable-content>
  <div></div>
  <svg></svg>

  <!-- These are "almost interactive": they could become interactive with the
  addition/removal of a non-tabindex attribute. -->
  <a></a>
  <audio></audio>
  <img>
  <input type=hidden>
  <video></video>

  <!-- These are considered interactive content for the purpose of <label> in a
  previous version of the HTML Standard, but no longer. -->
  <a tabindex=""></a>
  <audio tabindex=""></audio>
  <div tabindex=""></div>
  <img tabindex="">
  <input type=hidden tabindex="">
  <object></object>
  <object tabindex=""></object>
  <object usemap=""></object>
  <video tabindex=""></video>
</template>

<script>
"use strict";

const template = document.getElementById("noninteractive-unlabelable-content");
{
  const details = document.createElementNS("http://www.w3.org/2000/svg", "details");
  template.content.appendChild(details);
}

const elements = Array.from(template.content.children);
const label = document.getElementById("label");

for (const srcElement of elements) {
  test(t => {
    t.add_cleanup(() => {
      label.innerHTML = "";
    });

    const element = srcElement.cloneNode();
    label.appendChild(element);

    let clicked = 0;
    element.addEventListener("click", () => {
      clicked++;
    });
    element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    assert_equals(clicked, 1, "clicking interactive content");

    clicked = 0;
    const span = document.createElement("span");
    element.appendChild(span);
    span.click();
    assert_equals(clicked, 1, "clicking descendant of interactive content");
  }, `noninteractive unlabelable content ${srcElement.outerHTML} as first child of <label>`);

  test(t => {
    t.add_cleanup(() => {
      label.innerHTML = "";
    });

    const element = srcElement.cloneNode();
    const div = document.createElement("div");
    div.appendChild(element);
    label.appendChild(div);

    let clicked = 0;
    element.addEventListener("click", () => {
      clicked++;
    });
    element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    assert_equals(clicked, 1, "clicking nested interactive content");

    clicked = 0;
    const span = document.createElement("span");
    element.appendChild(span);
    span.click();
    assert_equals(clicked, 1, "clicking descendant of nested interactive content");
  }, `noninteractive unlabelable content ${srcElement.outerHTML} deeply nested under <label>`);

  test(t => {
    t.add_cleanup(() => {
      label.innerHTML = "";
    });

    const button = document.createElement("button");
    label.appendChild(button);

    const element = srcElement.cloneNode();
    label.appendChild(element);

    let buttonClicked = 0;
    button.addEventListener("click", () => {
      buttonClicked++;
    });

    let clicked = 0;
    element.addEventListener("click", () => {
      clicked++;
    });
    element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    assert_equals(clicked, 1, "clicking noninteractive unlabelable content");
    assert_equals(buttonClicked, 1, "clicking noninteractive unlabelable content should click button");

    buttonClicked = 0;
    clicked = 0;
    const span = document.createElement("span");
    element.appendChild(span);
    span.click();
    assert_equals(clicked, 1, "clicking descendant of nested noninteractive unlabelable content");
    assert_equals(
      buttonClicked, 1,
      "clicking descendant of nested noninteractive unlabelable content should click button"
    );
  }, `noninteractive unlabelable content ${srcElement.outerHTML} as second child under <label>`);
}

</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 522,
        "byte_start": 517,
        "col": 3,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 522,
        "byte_start": 517,
        "col": 3,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 801,
        "byte_start": 784,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 801,
        "byte_start": 784,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 846,
        "byte_start": 838,
        "col": 3,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 878,
        "byte_start": 858,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 908,
        "byte_start": 890,
        "col": 3,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 908,
        "byte_start": 890,
        "col": 3,
        "line": 29
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
  "source_name": "html/semantics/forms/the-label-element/clicking-noninteractive-unlabelable-content.html"
}
```
