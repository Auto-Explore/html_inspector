# html/cross-origin-embedder-policy/resources/blob-url-factory.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/resources/blob-url-factory.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<body>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="script-factory.js"></script>
<script>
const query = new URLSearchParams(window.location.search);
const id = query.get("id");
const variant = query.get("variant");
let parent = "parent";
let useDispatcher = false;

if (variant === "subframe") {
  parent = "parent.parent";
} else if (variant === "popup") {
  parent = "opener.parent";
} else if (variant === "popup-dispatch") {
  useDispatcher = true;
}

const blobContent = `
  <!doctype html>
  <base href="${window.location.href}">
  <script src="/common/utils.js"><\/script>
  <script src="/common/dispatcher/dispatcher.js"><\/script>

  <script>${createScript(window.origin, query.get("crossOrigin"), parent, id, useDispatcher)}<\/script>
`;
const blob = new Blob([blobContent], { type: "text/html" });
const blobURL = URL.createObjectURL(blob);
if (variant === "subframe") {
  const frame = document.createElement("iframe");
  frame.src = blobURL;
  document.body.append(frame);
} else if (variant === "popup" || variant === "popup-dispatch") {
  window.open(blobURL);
} else {
  window.location = blobURL;
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
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
  "source_name": "html/cross-origin-embedder-policy/resources/blob-url-factory.html"
}
```
