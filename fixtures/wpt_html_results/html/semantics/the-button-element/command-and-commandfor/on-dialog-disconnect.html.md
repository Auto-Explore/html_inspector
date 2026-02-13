# html/semantics/the-button-element/command-and-commandfor/on-dialog-disconnect.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-dialog-disconnect.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<meta name="timeout" content="long">
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/invoker-utils.js"></script>

<dialog id="invokee"></dialog>
<button id="invokerbutton" commandfor="invokee" command="show-modal"></button>

<script>
  const invokee = document.getElementById('invokee');
  test(
    function (t) {
      assert_false(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      let fired = false;
      invokee.addEventListener('command', () => {
        fired = true;
        invokee.remove();
      });
      invokerbutton.click();
      assert_true(fired, "command event fired");
      assert_false(invokee.isConnected, "dialog no longer connected");
      assert_false(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
    },
    `invoking a dialog and removing during command event does not raise an error`,
  );
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-dialog-disconnect.html"
}
```
