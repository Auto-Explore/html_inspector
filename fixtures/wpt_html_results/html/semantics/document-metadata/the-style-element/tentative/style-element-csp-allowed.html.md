# html/semantics/document-metadata/the-style-element/tentative/style-element-csp-allowed.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/tentative/style-element-csp-allowed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<meta charset="utf-8" />
<meta name="author" title="Kurt Catti-Schmidt" href="mailto:kschmi@microsoft.com" />
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#the-style-element" />
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/ShadowDOM/explainer.md" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<meta http-equiv="Content-Security-Policy" content="style-src 'self' 'unsafe-inline' data: blob:;">

<script>
  const t1 = async_test("Test securitypolicyviolation event doesn't fire on Declarative CSS Module when allowed via CSP");
  document.documentElement.addEventListener("securitypolicyviolation", t1.unreached_func("securitypolicyviolation error fired."));

  const t2 = async_test("Test error event doesn't fire on Declarative CSS Module when allowed via CSP");
</script>

<style type="module" specifier="foo" onerror="t2.unreached_func('onerror fired');">
    #test {color:blue}
</style>

</head>
<body>

<div id="test">Test content</div>

<script type="module">
  import sheet from "foo" with { type: "css"};

  test(function (t) {
    assert_equals(
      sheet.cssRules.length,
      1,
      "Declaratively defined rules were imported with `unsafe-inline` CSP.",
    );

    document.adoptedStyleSheets = [sheet];
    const test_element = document.getElementById("test");
    assert_equals(getComputedStyle(test_element)
              .color, "rgb(0, 0, 255)",
              "Declarative styles were applied.");

  }, "CSS Modules can be defined declaratively with `style-src` CSP set to `unsafe-inline` with the data protocol permitted.");
  t1.done();
  t2.done();
</script>
</body>
</html>
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
        "byte_end": 139,
        "byte_start": 55,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 139,
        "byte_start": 55,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.text_css_only",
      "message": "The only allowed value for the “type” attribute for the “style” element is “text/css” (with no parameters). (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 1020,
        "byte_start": 937,
        "col": 1,
        "line": 20
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
  "source_name": "html/semantics/document-metadata/the-style-element/tentative/style-element-csp-allowed.html"
}
```
