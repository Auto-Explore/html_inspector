# html/semantics/scripting-1/the-script-element/module/error-type-3.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/error-type-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of different types of errors</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));
    window.addEventListener("unhandledrejection", unreachable);

    const test_load = async_test(
        "instantiation error has higher priority than evaluation error");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 5);

      // An evaluation error is reported for the first top-level
      // <script> element for throw.js.
      assert_equals(log[0], 'throw');
      assert_true(log[1].foo);
      assert_equals(log[2], 1);

      // An instantiation error is reported for the second top-level <script>.
      assert_equals(log[3].constructor, SyntaxError);
      assert_equals(log[4], 2);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./throw.js"
    onerror="unreachable()" onload="log.push(1)"></script>
<script type="module" src="./error-type-3.js"
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/error-type-3.html"
}
```
