# html/semantics/selectors/pseudo-classes/link.html

Counts:
- errors: 6
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/link.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:link)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<link rel=stylesheet href="non-existent.css" id=link3>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<div id="log"></div>
<a id=link4></a>
<area id=link5></area>
<link id=link6></link>
<a href="http://www.w3.org" id=link7></a>
<area href="http://www.w3.org" id=link8></area>
<link href="http://www.w3.org" id=link9></link>
<a href="http://[" id=link10></a>

<script>
  testSelectorIdsMatch(":link", ["link7", "link8", "link10"], "Only <a>s and <area>s that have a href attribute match ':link'");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 497,
        "byte_start": 482,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “area”.",
      "severity": "Error",
      "span": {
        "byte_end": 504,
        "byte_start": 497,
        "col": 16,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 520,
        "byte_start": 505,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “link”.",
      "severity": "Error",
      "span": {
        "byte_end": 527,
        "byte_start": 520,
        "col": 16,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 610,
        "byte_start": 570,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “area”.",
      "severity": "Error",
      "span": {
        "byte_end": 617,
        "byte_start": 610,
        "col": 41,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 658,
        "byte_start": 618,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.link.in_body.disallowed_rel",
      "message": "A “link” element must not appear as a descendant of a “body” element unless the “link” element has an “itemprop” attribute or has a “rel” attribute whose value contains “dns-prefetch”, “modulepreload”, “pingback”, “preconnect”, “prefetch”, “preload”, “prerender”, or “stylesheet”.",
      "severity": "Warning",
      "span": {
        "byte_end": 658,
        "byte_start": 618,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “link”.",
      "severity": "Error",
      "span": {
        "byte_end": 665,
        "byte_start": 658,
        "col": 41,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.a.href.invalid",
      "message": "Bad value “http://[” for attribute “href” on element “a”.",
      "severity": "Warning",
      "span": {
        "byte_end": 695,
        "byte_start": 666,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/selectors/pseudo-classes/link.html"
}
```
