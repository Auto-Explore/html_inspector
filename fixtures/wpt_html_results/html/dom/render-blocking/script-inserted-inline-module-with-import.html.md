# html/dom/render-blocking/script-inserted-inline-module-with-import.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/script-inserted-inline-module-with-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<title>Script-inserted module script elements with "blocking=render" are render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  window.did_execute_script = false;
  const script = document.createElement("script");
  script.type = "module";
  script.blocking = "render";
  script.textContent = `
    import "/loading/resources/dummy.js?pipe=trickle(d1)";
    window.did_execute_script = true;
  `;
  document.head.append(script);
</script>
</head>
<div id="dummy">some text</div>

<script>
    promise_test(async t => {
      await new Promise(resolve => requestAnimationFrame(() => resolve()));
      assert_true(window.did_execute_script, "Script-inserted render-blocking inline module script should execute before rAF callback");
    });
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/render-blocking/script-inserted-inline-module-with-import.html"
}
```
