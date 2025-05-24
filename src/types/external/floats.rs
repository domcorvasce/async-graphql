use crate::{
    InputValueError, InputValueResult, Number, Scalar, ScalarType,
    Value::{self, String},
};

/// The `Float` scalar type represents signed double-precision fractional values as specified by [IEEE 754](https://en.wikipedia.org/wiki/IEEE_floating_point).
#[Scalar(internal, name = "Float")]
impl ScalarType for f32 {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::Number(n) => Ok(n
                .as_f64()
                .ok_or_else(|| InputValueError::from("Invalid number"))?
                as Self),

            Value::Object(n) => match n.get("$serde_json::private::Number") {
                Some(String(s)) => s
                    .parse::<f32>()
                    .map_err(|_| InputValueError::from("Invalid number")),
                _ => Err(InputValueError::from("Invalid number")),
            },

            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn is_valid(value: &Value) -> bool {
        match value {
            Value::Number(_) => true,
            Value::Object(obj) => {
                matches!(obj.get("$serde_json::private::Number"), Some(String(_)))
            }
            _ => false,
        }
    }

    fn to_value(&self) -> Value {
        match Number::from_f64(*self as f64) {
            Some(n) => Value::Number(n),
            None => Value::Null,
        }
    }
}

/// The `Float` scalar type represents signed double-precision fractional values as specified by [IEEE 754](https://en.wikipedia.org/wiki/IEEE_floating_point).
#[Scalar(internal, name = "Float")]
impl ScalarType for f64 {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::Number(n) => Ok(n
                .as_f64()
                .ok_or_else(|| InputValueError::from("Invalid number"))?
                as Self),

            Value::Object(n) => match n.get("$serde_json::private::Number") {
                Some(String(s)) => s
                    .parse::<f64>()
                    .map_err(|_| InputValueError::from("Invalid number")),
                _ => Err(InputValueError::from("Invalid number")),
            },

            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn is_valid(value: &Value) -> bool {
        match value {
            Value::Number(_) => true,
            Value::Object(obj) => {
                matches!(obj.get("$serde_json::private::Number"), Some(String(_)))
            }
            _ => false,
        }
    }

    fn to_value(&self) -> Value {
        match Number::from_f64(*self) {
            Some(n) => Value::Number(n),
            None => Value::Null,
        }
    }
}
