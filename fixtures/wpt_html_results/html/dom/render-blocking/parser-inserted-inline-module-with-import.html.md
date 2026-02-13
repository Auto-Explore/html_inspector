# html/dom/render-blocking/parser-inserted-inline-module-with-import.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/parser-inserted-inline-module-with-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<title>Parser-inserted module script elements with "blocking=render" are render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  window.did_execute_script = false;
</script>
<script type="module" blocking="render">
  import "/loading/resources/dummy.js?pipe=trickle(d1)";
  window.did_execute_script = true;
</script>
</head>
<div id="dummy">some text</div>

<script>
    promise_test(async t => {
      await new Promise(resolve => requestAnimationFrame(() => resolve()));
      assert_true(window.did_execute_script, "Parser-inserted render-blocking inline module script should execute before rAF callback");
    });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.module.inline.blocking.disallowed",
      "message": "An inline “script” element with “type=module” must not have a “blocking” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 322,
        "byte_start": 282,
        "col": 1,
        "line": 9
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
  "source_name": "html/dom/render-blocking/parser-inserted-inline-module-with-import.html"
}
```
