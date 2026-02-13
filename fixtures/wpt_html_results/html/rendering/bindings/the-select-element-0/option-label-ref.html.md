# html/rendering/bindings/the-select-element-0/option-label-ref.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/bindings/the-select-element-0/option-label-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Option labels</title>
<select size=12>
  <option><!-- No children, no label-->
  <option><!-- No children, empty label-->
  <option>label<!-- No children, label-->
  <option><!-- No children, namespaced label-->

  <option>child<!-- Single child, no label-->
  <option>child<!-- Single child, empty label-->
  <option>label<!-- Single child, label-->
  <option>child<!-- Single child, namespaced label-->

  <option>child node<!-- Two children, no label-->
  <option>child node<!-- Two children, empty label-->
  <option>label<!-- Two children, label-->
  <option>child node<!-- Two children, namespaced label-->
</select>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
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
  "source_name": "html/rendering/bindings/the-select-element-0/option-label-ref.html"
}
```
