# html/semantics/document-metadata/interactions-of-styling-and-scripting/conditionally-block-rendering-on-link-media-attr.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/interactions-of-styling-and-scripting/conditionally-block-rendering-on-link-media-attr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="support/utils.js"></script>

<link rel=stylesheet href=stylesheet.py>
<link rel=stylesheet media="screen and (max-width:10px)" href=stylesheet.py?stylesNotMatchingEnvironment&delay=2>
<h1>Dominic Farolino</h1>
<script>
  test(() => {
    const h1 = document.querySelector('h1');
    const computedColor = getComputedStyle(h1).color;
    const expectedColor = "rgb(128, 0, 128)";

    assert_equals(computedColor, expectedColor);
    assert_true(styleExists("h1 { color: purple; }")); // first style sheet
    assert_false(styleExists("h1 { color: brown; }")); // second style sheet (should not be loaded yet)
  }, "Only the style sheet loaded via a link element whose media attribute matches the environment should block following script execution");

  const secondStylesheetTest = async_test("Both style sheets loaded via the link elements should be registered as style sheets for the document after 2 seconds");
  secondStylesheetTest.step_timeout(() => {
      assert_true(styleExists("h1 { color: purple; }")); // first style sheet
      assert_true(styleExists("h1 { color: brown; }")); // second style sheet (loaded now!)
      secondStylesheetTest.done();
  }, 3000);
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
  "source_name": "html/semantics/document-metadata/interactions-of-styling-and-scripting/conditionally-block-rendering-on-link-media-attr.html"
}
```
