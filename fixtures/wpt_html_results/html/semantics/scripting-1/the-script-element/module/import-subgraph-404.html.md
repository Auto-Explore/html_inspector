# html/semantics/scripting-1/the-script-element/module/import-subgraph-404.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/import-subgraph-404.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="module">
import { delayedLoaded }  from "./resources/delayed-modulescript.py";
import { A } from "./404.js";
window.loadSuccess = delayedLoaded;
</script>
<script type="module">
test(function () {
    assert_equals(window.loadSuccess, undefined,
      "module tree w/ its sub graph 404 should fail to load without crashing");
}, "Import a module graph w/ sub-graph 404.");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/import-subgraph-404.html"
}
```
