# html/semantics/scripting-1/the-script-element/module/choice-of-error-3.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/choice-of-error-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Choice of evaluation errors</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));
    window.addEventListener("unhandledrejection", unreachable);

    const test_load = async_test(
        "Evaluation errors are cached in intermediate module scripts");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 5);

      // Evaluation errors, unlike parse/instantiation errors, are remembered
      // and cached in module scripts between the root and the script that
      // caused an evaluation error, and thus the same evaluation error
      // is reported for both <script> elements.
      assert_equals(log[0], "throw2");
      assert_true(log[1].bar);
      assert_equals(log[2], 1);

      assert_true(log[3].bar);
      assert_equals(log[4], 2);

      assert_equals(log[1], log[3], 'evaluation errors must be the same');
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./choice-of-error-3a.js"
    onerror="unreachable()" onload="log.push(1)"></script>
<script type="module" src="./choice-of-error-3b.js"
    onerror="unreachable()" onload="log.push(2)"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/choice-of-error-3.html"
}
```
