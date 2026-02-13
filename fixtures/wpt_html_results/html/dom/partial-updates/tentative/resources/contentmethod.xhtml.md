# html/dom/partial-updates/tentative/resources/contentmethod.xhtml

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/resources/contentmethod.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
<html>
<section xmlns="http://www.w3.org/1999/xhtml" id="placeholder" contentname="p">Unchanged</section>
<template xmlns="http://www.w3.org/1999/xhtml" id="patch" contentmethod="replace-children">
    <section contentname="p"></section>
</template>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.doctype.not_html5",
      "message": "Obsolete doctype. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 203,
        "byte_start": 193,
        "col": 89,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 335,
        "byte_start": 325,
        "col": 30,
        "line": 5
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
  "source_name": "html/dom/partial-updates/tentative/resources/contentmethod.xhtml"
}
```
