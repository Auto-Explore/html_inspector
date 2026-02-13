# html/semantics/scripting-1/the-script-element/module/currentScript-null.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/currentScript-null.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="module" src="set-currentScript-on-window.js"></script>
<script type="module">
import { currentScriptOnImportedModule } from "./currentscript.js";

test(() => {
  assert_equals(document.currentScript, null, "document.currentScript on inline scripts should be null");
  assert_equals(currentScriptOnImportedModule, null, "document.currentScript on imported scripts should be null");
  assert_equals(window.currentScriptRecorded, null, "document.currentScript on external module scripts should be null");
}, "currentScript on script type=module should be all null");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/currentScript-null.html"
}
```
