# html/semantics/scripting-1/the-script-element/module/error-type-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/error-type-2.html",
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
        "parse error has higher priority than instantiation error");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 4);

      // An instantiation error is reported for the first top-level
      // <script> element for instantiation-error-1.js.
      assert_equals(log[0].constructor, SyntaxError);
      assert_equals(log[1], 1);

      // A parse error is reported for the second top-level <script>.
      assert_equals(log[2].constructor, SyntaxError);
      assert_equals(log[3], 2);
      assert_not_equals(log[0], log[2]);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./instantiation-error-1.js"
    onerror="unreachable()" onload="log.push(1)"></script>
<script type="module" src="./error-type-2.js"
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/error-type-2.html"
}
```
