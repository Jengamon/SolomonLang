{{ imports }}

#[derive(Clone, Debug)]
pub enum {{data_type | capitalize}} {
  {% for type in types -%}
    {{type.name}}({{type.name}}),
{% endfor -%}
}

impl {{data_type | capitalize}} {
  {% for type in types -%}

  pub fn is_{{type.name | downcase}}(&self) -> bool {
    match self {
      &{{data_type | capitalize}}::{{type.name}}(..) => true,
      _ => false,
    }
  }

  pub fn as_{{type.name | downcase}}(&self) -> Option<{{type.name}}> {
    match self {
      &{{data_type | capitalize}}::{{type.name}}(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  {% endfor %}
}

{% for type in types %}
#[derive(Clone, Debug)]
pub struct {{type.name}} {
  {% for member in type.members -%}
    pub {{member.name}}: {{member.type}},
  {% endfor %}
}

impl {{type.name}} {
  pub fn new_naked(
  {%- for member in type.members -%}
    {{member.name}}: {{member.type}},
  {%- endfor -%}
  ) -> {{type.name}} {
    {{type.name}} {
      {%- for member in type.members %}
        {{member.name}},
      {% endfor %}
    }
  }

  pub fn boxed_new_naked(
  {%- for member in type.members -%}
    {{member.name}}: {{member.type}},
  {%- endfor -%}
  ) -> Box<{{type.name}}> {
    Box::new({{type.name}} {
      {%- for member in type.members %}
        {{member.name}},
      {% endfor %}
    })
  }

  pub fn new(
  {%- for member in type.members -%}
    {{member.name}}: {{member.type}},
  {%- endfor -%}
  ) -> {{data_type | capitalize}} {
    {{data_type | capitalize}}::{{type.name}}({{type.name}}{
      {%- for member in type.members %}
        {{member.name}},
      {% endfor %}
    })
  }

  pub fn boxed_new(
  {%- for member in type.members -%}
    {{member.name}}: {{member.type}},
  {%- endfor -%}
  ) -> Box<{{data_type | capitalize}}> {
    Box::new({{data_type | capitalize}}::{{type.name}}({{type.name}} {
      {%- for member in type.members %}
        {{member.name}},
      {% endfor %}
    }))
  }
}
{% endfor %}

pub trait {{data_type | capitalize}}Visitor<N> {
  {% for type in types -%}
  fn visit_{{type.name | downcase}}(&self, {{type.name | downcase}}: {{type.name}}) -> N;
  {% endfor %}

  fn visit_{{data_type}}(&self, d: {{data_type | capitalize}}) -> N {
    match d {
      {% for type in types -%}
      {{data_type | capitalize}}::{{type.name}}(ref d) => self.visit_{{type.name | downcase}}(d.clone()),
      {% endfor %}
    }
  }

  fn visit_{{data_type}}_box(&self, d: Box<{{data_type | capitalize}}>) -> N {
    match &*d {
      {% for type in types -%}
      &{{data_type | capitalize}}::{{type.name}}(ref d) => self.visit_{{type.name | downcase}}(d.clone()),
      {% endfor %}
    }
  }
}

pub trait {{data_type | capitalize}}MutVisitor<N> {
  {% for type in types -%}
  fn visit_{{type.name | downcase}}(&mut self, {{type.name | downcase}}: {{type.name}}) -> N;
  {% endfor %}

  fn visit_{{data_type}}(&mut self, d: {{data_type | capitalize}}) -> N {
    match d {
      {% for type in types -%}
      {{data_type | capitalize}}::{{type.name}}(ref d) => self.visit_{{type.name | downcase}}(d.clone()),
      {% endfor %}
    }
  }

  fn visit_{{data_type}}_box(&mut self, d: Box<{{data_type | capitalize}}>) -> N {
    match &*d {
      {% for type in types -%}
      &{{data_type | capitalize}}::{{type.name}}(ref d) => self.visit_{{type.name | downcase}}(d.clone()),
      {% endfor %}
    }
  }
}
