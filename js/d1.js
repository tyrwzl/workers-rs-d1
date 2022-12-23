export const fetch_customers = async (env, query, customer_id) => {
	const { results } = await env.DB.prepare(query).bind(customer_id).all();
	return JSON.stringify(results);
};
