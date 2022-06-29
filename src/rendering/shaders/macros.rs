#[macro_export]
macro_rules! vertex_attr {
    ($offset:expr => $format:ident [$location:expr]) => {
        VertexAttribute {
            format: AttributeFormat::$format,
            offset: $offset,
            location: $location,
        }
    };
}

#[macro_export]
macro_rules! vertex_attrs_exact {
    [] => {
        []
    };

    [$offset:expr => $format:ident [$location:expr]] => {
        [
            $crate::VertexAttribute {
                format: AttributeFormat::$format,
                offset: $offset,
                location: $location,
            }
        ]
    };

    [$($offset:expr => $format:ident [$location:expr]),* $(,)?] => {
        [$(
            VertexAttribute {
                format: AttributeFormat::$format,
                offset: $offset,
                location: $location,
            },
        )*]
    };
}

#[macro_export]
macro_rules! vertex_attrs {
    [] => {
        []
    };

    [$($format:ident),* $(,)?] => {
        $crate::vertex_attrs!(@parse $($format,)*)
    };

    (@parse $format:ident) => {
        $crate::vertex_attrs!(
            @parse_end
            [0u64 => $format [0u32]]
        )
    };

    (@parse $format:ident, $($next_format:ident,)*) => {
        $crate::vertex_attrs!(
            @parse
            [];
            0u64 => $format [0u32];
            $($next_format,)*
        )
    };

    (@parse [$($offset:expr => $format:ident [$location:expr],)*]) => {
        $crate::vertex_attrs_exact![$($offset => $format [$location],)*]
    };

    (@parse
        [$($prev_offset:expr => $prev_format:ident [$prev_location:expr],)*];
        $offset:expr => $format:ident [$location:expr];
        $next_format:ident,
        $($tail_formats:ident,)*
    ) => {
        vertex_attrs!(
            @parse
            [
                $($prev_offset => $prev_format [$prev_location],)*
                $offset => $format [$location],
            ];
            $offset + AttributeFormat::$format.size() => $next_format [$location + 1u32];
            $($tail_formats,)*
        )
    };

    (@parse
        [$($prev_offset:expr => $prev_format:ident [$prev_location:expr],)*];
        $offset:expr => $format:ident [$location:expr];
    ) => {
        vertex_attrs!(
            @parse
            [
                $($prev_offset => $prev_format [$prev_location],)*
                $offset => $format [$location],
            ]
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::rendering::shaders::{
        AttributeFormat,
        VertexAttribute,
    };

    #[test]
    fn vertex_attrs_exact_macro_test() {
        let a = vertex_attrs_exact![
            0  => Float32x2 [0],
        ];

        let b = vertex_attrs_exact![
            0  => Float32x2 [0],
            8  => Float32x4 [1],
            24 => Float32x4 [2],
        ];

        assert_eq!(
            a,
            [
                VertexAttribute { format: AttributeFormat::Float32x2, offset: 0, location: 0 },
            ]
        );

        assert_eq!(
            b,
            [
                VertexAttribute { format: AttributeFormat::Float32x2, offset: 0,  location: 0 },
                VertexAttribute { format: AttributeFormat::Float32x4, offset: 8,  location: 1 },
                VertexAttribute { format: AttributeFormat::Float32x4, offset: 24, location: 2 },
            ]
        );
    }

    #[test]
    fn vertex_attributes_macro_test() {
        let a = vertex_attrs![
            Float32x2,
        ];

        let b = vertex_attrs![
            Float32x2,
            Float32x4,
            Float32x4,
        ];

        assert_eq!(
            a,
            [
                VertexAttribute { format: AttributeFormat::Float32x2, offset: 0, location: 0 },
            ]
        );

        assert_eq!(
            b,
            [
                VertexAttribute { format: AttributeFormat::Float32x2, offset: 0,  location: 0 },
                VertexAttribute { format: AttributeFormat::Float32x4, offset: 8,  location: 1 },
                VertexAttribute { format: AttributeFormat::Float32x4, offset: 24, location: 2 },
            ]
        );

        //

        let exact_a = vertex_attrs_exact![
            0  => Float32x2 [0],
        ];

        let exact_b = vertex_attrs_exact![
            0  => Float32x2 [0],
            8  => Float32x4 [1],
            24 => Float32x4 [2],
        ];

        assert_eq!(a, exact_a);
        assert_eq!(b, exact_b);
    }
}
