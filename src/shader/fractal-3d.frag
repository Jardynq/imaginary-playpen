#version 300 es
precision highp float;
precision highp int;

uniform float time;

in vec2 f_uv;
out vec4 color;


float sdf_mandebulp(vec3 ray, vec3 origin) {
    float scale = 2.0;
    float factor = scale;

    float radius = 1.0;
    radius = radius * radius;
    float min_radius = 0.5;
    min_radius = min_radius * min_radius;

    float x = origin.x;
    float y = origin.y;
    float z = origin.z;

    for (int i = 0; i < 100; i++) {
        if (x > 1.0) {
            x = 2.0 - x;
        } else if (x < -1.0) {
            x = -2.0 - x;
        }

        if (y > 1.0) {
            y = 2.0 - y;
        } else if (y < -1.0) {
            y = -2.0 - y;
        }

        if (z > 1.0) {
            z = 2.0 - z;
        } else if (z < -1.0) {
            z = -2.0 - z;
        } 

        float mag = x * x + y * y + z * z;

        if (mag < min_radius) {
            x = x * radius / min_radius;
            y = y * radius / min_radius;
            z = z * radius / min_radius;
            factor = factor * radius / min_radius;
        }
        else if (mag < radius) {
            x = x * radius / mag;
            y = y * radius / mag;
            z = z * radius / mag;
            factor *= radius / mag;
        }

        x = x * scale + ray.x;
        y = y * scale + ray.y;
        z = z * scale + ray.z;
        factor *= scale;

    }
    return sqrt(x * x + y * y + z * z) / abs(factor);
}
vec2 DE(vec3 pos) {
    float Bailout = 20.0;
    int Iterations = 50;
    float Power = 8.0;

	vec3 z = pos;
	float dr = 1.0;
	float r = 0.0;
    int i = 0;
	for (; i < Iterations ; i++) {
		r = length(z);
		if (r>Bailout) break;
		
		// convert to polar coordinates
		float theta = acos(z.z/r);
		float phi = atan(z.y,z.x);
		dr =  pow( r, Power-1.0)*Power*dr + 1.0;
		
		// scale and rotate the point
		float zr = pow( r,Power);
		theta = theta*Power;
		phi = phi*Power;
		
		// convert back to cartesian coordinates
		z = zr*vec3(sin(theta)*cos(phi), sin(phi)*sin(theta), cos(theta));
		z+=pos;
	}
	return vec2(0.5*log(r)*r/dr, i);
}

float sdf_sphere(vec3 ray, vec3 origin, float radius) {
    return distance(ray, origin) - radius;
}


vec3 shader_phong(vec3 normal, vec3 light, float distance, float power, vec3 color) {
    return clamp(dot(light, normal), 0.0, 1.0) * color * power / distance;
}


void main() {
    float escape_radius = 0.000001;

    vec3 ray = vec3(f_uv.x - 0.5, f_uv.y - 0.5, -5.0);
    vec3 direction = vec3(f_uv.x - 0.5, f_uv.y - 0.5, 1.0);

    vec3 light = vec3(cos(time), (sin(time) + 1.0 / 2.0) * 6.0, 5.0);
    vec3 sphere = vec3(0.0, 0.0, 5.0);

    bool is_hit = false;
    float dist = 0.0;
    float iter = 0.0;
    for (int i = 0; i < 500; i++) {
        vec2 temp = DE(ray);
        dist = temp.x;
        iter = temp.y;
        if (dist < escape_radius) {
            is_hit = true;
            break;
        }
        ray += direction * dist;
    }

    if (is_hit) {
        //vec3 intens = shader_phong(normalize(sphere - ray), light, distance(ray, light), 10.0, vec3(0.0, 1.0, 0.0));
        //color = vec4(0.5, 0.1, 0.1, 1.0) * vec4(intens.x, intens.y, intens.z, 1.0);
        color = vec4(iter / 50.0, 0.1, 0.1, 1.0);
    } else {
        color = vec4(0.0, 0.0, 1.0, 1.0);

    }
    //color = f_uv;
}
