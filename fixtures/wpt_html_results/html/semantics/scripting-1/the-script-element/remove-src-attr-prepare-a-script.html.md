# html/semantics/scripting-1/the-script-element/remove-src-attr-prepare-a-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/remove-src-attr-prepare-a-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<link rel=help href=https://github.com/whatwg/html/pull/10188/files#r1685905457>
<title>Remove src attribute does not "prepare the script"</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
test(() => {
  // Flags that the script element in this test will change, if it incorrectly
  // executes.
  window.didExecute = false;
  window.innerTextExecuted = false;

  const script = document.createElement('script');
  // Invalid type, so the script won't execute upon insertion.
  script.type = 'invalid';
  script.src = 'resources/flag-setter.js';
  script.innerText = 'window.innerTextExecuted = true';
  document.body.append(script);
  assert_false(window.didExecute);
  assert_false(window.innerTextExecuted);

  // Make script valid, but don't immediately execute it.
  script.type = '';

  // Removing the `src` content attribute does not trigger the "prepare a
  // script" algorithm on the script.
  script.removeAttribute('src');
  assert_false(window.didExecute);
  assert_false(window.innerTextExecuted);
}, "Removing the `src` content attribute does not 'prepare' the script");
</script>
</body>
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
  "source_name": "html/semantics/scripting-1/the-script-element/remove-src-attr-prepare-a-script.html"
}
```
