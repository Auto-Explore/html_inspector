# html/semantics/document-metadata/the-link-element/link-rel-attribute.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-rel-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src = "/resources/testharness.js"></script>
<script src = "/resources/testharnessreport.js"></script>

<link id="light-link" rel="stylesheet" href="resources/link-rel-attribute.css">
<div id="light-div" class="green">I"m green when light DOM link is on</div>

<div id="host">
  I"m green when Shadow DOM link is on
  <template id="shadow-dom">
    <link id="shadow-link" rel="stylesheet" href="resources/link-rel-attribute.css">
    <div id="shadow-div" class="green">
      <slot></slot>
    </div>
  </template>
</div>

<script>

function testLinkRelModification(testDiv, testLink) {
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 128, 0)");
  testLink.setAttribute("rel", "no-stylesheet");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 0, 0)");
  testLink.setAttribute("rel", "stylesheet");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 128, 0)");
  testLink.removeAttribute("rel");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 0, 0)");
}

test (() => {
  testLinkRelModification(document.querySelector("#light-div"),
                          document.querySelector("#light-link"));
}, "Removing stylesheet from link rel attribute should remove the stylesheet for light DOM");

test (() => {
  var host = document.querySelector("#host");
  var shadow = host.attachShadow({ mode: "open" });
  var tmpl = document.querySelector("template#shadow-dom");
  var clone = document.importNode(tmpl.content, true);
  shadow.appendChild(clone);
  testLinkRelModification(shadow.querySelector("#shadow-div"),
                          shadow.querySelector("#shadow-link"));
}, "Removing stylesheet from link rel attribute should remove the stylesheet for shadow DOM");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-link-element/link-rel-attribute.html"
}
```
