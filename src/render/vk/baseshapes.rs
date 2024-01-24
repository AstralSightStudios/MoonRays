use super::GlslVertex;

pub struct BaseIndices{

}

impl BaseIndices{
    pub fn RECTANGLE() -> Vec<u32>{
        let ret = vec![
            0, 1, 2, 2, 3, 0
        ];

        return ret;
    }

    pub fn TRIANGLE() -> Vec<u32>{
        let ret = vec![
            0, 1, 2
        ];

        return ret;
    }
}

pub struct BaseVertices{
    
}

impl BaseVertices{
    pub fn RECTANGLE() -> Vec<GlslVertex::GlslVertexBase>{
        let ret = vec![
            GlslVertex::GlslVertexBase{
                pos: glm::vec2(-0.5, -0.5), 
                color: glm::vec3(1.0, 0.0, 0.0)
            },
            GlslVertex::GlslVertexBase{
                pos: glm::vec2(0.5, -0.5), 
                color: glm::vec3(0.0, 1.0, 0.0)
            },
            GlslVertex::GlslVertexBase{
                pos: glm::vec2(0.5, 0.5), 
                color: glm::vec3(0.0, 0.0, 1.0)
            },
            GlslVertex::GlslVertexBase{
                pos: glm::vec2(-0.5, 0.5), 
                color: glm::vec3(1.0, 1.0, 1.0)
            }
        ];
        
        return ret;
    }

    pub fn TRIANGLE() -> Vec<GlslVertex::GlslVertexBase>{
        let ret = vec![
            GlslVertex::GlslVertexBase{
                pos: glm::vec2(0.0, -0.5), 
                color: glm::vec3(1.0, 0.0, 0.0)
            },
            GlslVertex::GlslVertexBase{
                pos: glm::vec2(0.5, 0.5), 
                color: glm::vec3(0.0, 1.0, 0.0)
            },
            GlslVertex::GlslVertexBase{
                pos: glm::vec2(-0.5, 0.5), 
                color: glm::vec3(0.0, 0.0, 1.0)
            },
        ];

        return ret;
    }
}