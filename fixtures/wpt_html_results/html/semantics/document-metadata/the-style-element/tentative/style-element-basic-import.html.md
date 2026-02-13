# html/semantics/document-metadata/the-style-element/tentative/style-element-basic-import.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/tentative/style-element-basic-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Kurt Catti-Schmidt" href="mailto:kschmi@microsoft.com" />
<meta name="timeout" content="long" />
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#the-style-element" />
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/ShadowDOM/explainer.md" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style type="module" specifier="foo">
    #test {color:blue}
</style>

<div id="test">Test content</div>

<script type="module">
  import sheet from "foo" with { type: "css"};

  test(function (t) {
    assert_equals(
      sheet.cssRules.length,
      1,
      "Declaratively defined rules were imported imperatively.",
    );

    document.adoptedStyleSheets = [sheet];
    const test_element = document.getElementById("test");
    assert_equals(getComputedStyle(test_element)
              .color, "rgb(0, 0, 255)",
              "Declarative styles were applied.");

  }, "CSS Modules can be defined declaratively.");
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
        "byte_end": 125,
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
        "byte_end": 125,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.text_css_only",
      "message": "The only allowed value for the “type” attribute for the “style” element is “text/css” (with no parameters). (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 518,
        "byte_start": 481,
        "col": 1,
        "line": 10
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
  "source_name": "html/semantics/document-metadata/the-style-element/tentative/style-element-basic-import.html"
}
```
