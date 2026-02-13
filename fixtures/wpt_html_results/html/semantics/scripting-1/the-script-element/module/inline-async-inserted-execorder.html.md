# html/semantics/scripting-1/the-script-element/module/inline-async-inserted-execorder.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/inline-async-inserted-execorder.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Inline async="" module scripts execute or throw parse errors asynchronously</title>
<link rel="help" href="https://github.com/whatwg/html/issues/9864">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";
setup({ allow_uncaught_exception: true });

promise_test(async t => {
  window.log = ["before any script execution"];

  window.addEventListener("error", ev => {
    window.log.push("error event on Window");
  });

  const noErrorScript = document.createElement("script");
  noErrorScript.type = "module";
  noErrorScript.async = true;
  noErrorScript.textContent = "window.log.push('no error script executed');";

  // This should queue a task to run the script, but not run it immediately.
  document.head.append(noErrorScript);

  log.push("after inserting noErrorScript");
  assert_array_equals(window.log, [
    "before any script execution",
    "after inserting noErrorScript"
  ]);

  const parseErrorScript = document.createElement("script");
  parseErrorScript.type = "module";
  parseErrorScript.async = true;
  parseErrorScript.textContent = "+++++";

  // This should queue a task to fire the error event, but not fire it immediately.
  document.head.append(parseErrorScript);

  log.push("after inserting parseErrorScript");
  assert_array_equals(window.log, [
    "before any script execution",
    "after inserting noErrorScript",
    "after inserting parseErrorScript"
  ]);

  // After a microtask, the script run / error event must not happen.
  queueMicrotask(t.step_func(() => {
    assert_array_equals(window.log, [
      "before any script execution",
      "after inserting noErrorScript",
      "after inserting parseErrorScript"
    ]);
  }));

  // But it must eventually happen, after a full task.
  await t.step_wait(() => window.log.length == 5, "5 items must eventually be logged");

  // And when it does the order must be correct.
  assert_array_equals(window.log, [
    "before any script execution",
    "after inserting noErrorScript",
    "after inserting parseErrorScript",
    "no error script executed",
    "error event on Window"
  ]);
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/inline-async-inserted-execorder.html"
}
```
