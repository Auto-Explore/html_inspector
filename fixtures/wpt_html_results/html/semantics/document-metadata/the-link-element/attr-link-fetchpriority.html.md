# html/semantics/document-metadata/the-link-element/attr-link-fetchpriority.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/attr-link-fetchpriority.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Fetch Priority - Link element</title>
<meta name="author" title="Dominic Farolino" href="mailto:domfarolino@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<link id=link1 href=resources/stylesheet.css fetchpriority=high>
<link id=link2 href=resources/stylesheet.css fetchpriority=low>
<link id=link3 href=resources/stylesheet.css fetchpriority=auto>
<link id=link4 href=resources/stylesheet.css fetchpriority=xyz>
<link id=link5 href=resources/stylesheet.css>

<script>
  test(() => {
    assert_equals(link1.fetchPriority, "high", "high fetchPriority is a valid IDL value on the link element");
    assert_equals(link2.fetchPriority, "low", "low fetchPriority is a valid IDL value on the link element");
    assert_equals(link3.fetchPriority, "auto", "auto fetchPriority is a valid IDL value on the link element");
    assert_equals(link4.fetchPriority, "auto", "invalid fetchPriority reflects as 'auto' IDL attribute on the link element");
    assert_equals(link5.fetchPriority, "auto", "missing fetchPriority reflects as 'auto' IDL attribute on the link element");
  }, "fetchpriority attribute on <link> elements should reflect valid IDL values");

  test(() => {
    const link = document.createElement("link");
    assert_equals(link.fetchPriority, "auto");
  }, "default fetchpriority attribute on <link> elements should be 'auto'");
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
        "byte_end": 142,
        "byte_start": 61,
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
        "byte_end": 142,
        "byte_start": 61,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 314,
        "byte_start": 250,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 378,
        "byte_start": 315,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 443,
        "byte_start": 379,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 507,
        "byte_start": 444,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 553,
        "byte_start": 508,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-link-element/attr-link-fetchpriority.html"
}
```
